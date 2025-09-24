use axum::{routing::post, Router, Json};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signature::Signer;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

use solana_tss::tss;
use solana_tss::serialization::Serializes;
use solana_tss::serialization::AggMessage1;
use solana_tss::serialization::SecretAggStepOne;
use solana_tss::serialization::PartialSignature;

use solana_client::rpc_client::RpcClient;

use solana_sdk::hash::Hash;


#[derive(Serialize)]
struct GenerateResponse {
    secret_share: String,
    public_share: String,
}

async fn generate_handler() -> Json<GenerateResponse> {
let keypair = Keypair::generate(&mut rand07::thread_rng());

    Json(GenerateResponse {
        secret_share: keypair.to_base58_string(),
        public_share: keypair.pubkey().to_string(),
    })
}

#[derive(Deserialize)]
struct AggregateKeysRequest {
    keys: Vec<String>,
}

#[derive(Serialize)]
struct AggregateKeysResponse {
    aggregated_key: String,
}

async fn aggregate_keys_handler(
    Json(payload): Json<AggregateKeysRequest>,
) -> Json<AggregateKeysResponse> {
    // Convert Vec<String> -> Vec<Pubkey>
    let pubkeys: Vec<Pubkey> = payload
        .keys
        .iter()
        .map(|k| Pubkey::from_str(k).expect("Invalid pubkey"))
        .collect();

    // Pass pubkeys to tss::key_agg
    let aggkey = tss::key_agg(pubkeys, None).unwrap();
    let aggpubkey = Pubkey::new(&*aggkey.agg_public_key.to_bytes(true));

    Json(AggregateKeysResponse {
        aggregated_key: aggpubkey.to_string(),
    })
}

#[derive(Serialize)]
struct StepOneResponse {
    first_msg: String,
    secret_state: String,
}

async fn agg_send_step_one_handler() -> Json<StepOneResponse> {
    let keypair = Keypair::generate(&mut rand07::thread_rng());

    let (first_msg, secret) = tss::step_one(keypair);

    Json(StepOneResponse {
        first_msg: first_msg.serialize_bs58(),
        secret_state: secret.serialize_bs58(), 
    })
}

#[derive(Serialize)]
struct StepTwoResponse {
    partial_signature: String,
}

#[derive(Deserialize)]
struct StepTwoRequest {
    keypair: String,
    amount: f64,
    to: String,
    memo: Option<String>,
    recent_block_hash: String,
    keys: Vec<String>,
    first_messages: Vec<String>,
    secret_state: String,
}

async fn agg_send_step_two_handler(
    Json(payload): Json<StepTwoRequest>,
) -> Result<Json<StepTwoResponse>, String> {
    // Deserialize the inputs
    let keypair = Keypair::from_base58_string(&payload.keypair);
    let to = Pubkey::from_str(&payload.to).map_err(|e| e.to_string())?;
    let recent_block_hash = Hash::from_str(&payload.recent_block_hash).map_err(|e| e.to_string())?;
    let keys: Vec<Pubkey> = payload.keys
        .iter()
        .map(|k| Pubkey::from_str(k).map_err(|e| e.to_string()))
        .collect::<Result<_, _>>()?;
    let first_messages: Vec<AggMessage1> = payload.first_messages
        .iter()
        .map(|m| AggMessage1::deserialize_bs58(m).map_err(|e| e.to_string()))
        .collect::<Result<_, _>>()?;
    let secret_state = SecretAggStepOne::deserialize_bs58(&payload.secret_state).map_err(|e| e.to_string())?;

    let sig = tss::step_two(
        keypair,
        payload.amount,
        to,
        payload.memo,
        recent_block_hash,
        keys,
        first_messages,
        secret_state,
    ).map_err(|e| format!("StepTwo failed: {}", e))?;

    Ok(Json(StepTwoResponse {
        partial_signature: sig.serialize_bs58(),
    }))
}

#[derive(Serialize)]
struct BroadcastResponse {
    txn_id: String,
}


#[derive(Deserialize)]
struct BroadcastRequest {
    signatures: Vec<String>,
    amount: f64,
    to: String,
    memo: Option<String>,
    recent_block_hash: String,
    net: String,
    keys: Vec<String>,
}

async fn agg_sig_and_broadcast(
    Json(payload): Json<BroadcastRequest>,
) -> Result<Json<BroadcastResponse>, String> {
    // Deserialize the inputs
    let to = Pubkey::from_str(&payload.to).map_err(|e| e.to_string())?;
    let recent_block_hash = Hash::from_str(&payload.recent_block_hash).map_err(|e| e.to_string())?;
    let keys: Vec<Pubkey> = payload.keys
        .iter()
        .map(|k| Pubkey::from_str(k).map_err(|e| e.to_string()))
        .collect::<Result<_, _>>()?;
    let signatures: Vec<PartialSignature> = payload.signatures
        .iter()
        .map(|s| PartialSignature::deserialize_bs58(s).map_err(|e| e.to_string()))
        .collect::<Result<_, _>>()?;

    let rpc_client = RpcClient::new(payload.net.clone());

    let txn = tss::sign_and_broadcast(
        payload.amount,
        to,
        payload.memo,
        recent_block_hash,
        keys,
        signatures
    ).map_err(|e| format!("Broadcast failed: {}", e))?;

    let sig = rpc_client.send_transaction(&txn).map_err(|e| format!("Failed Sending Transaction: {}", e))?;

    Ok(Json(BroadcastResponse {
        txn_id: sig.to_string()
    }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/generate", post(generate_handler))
        .route("/aggregate_keys", post(aggregate_keys_handler))
        .route("/agg_send_step_one", post(agg_send_step_one_handler))
        .route("/agg_send_step_two", post(agg_send_step_two_handler))
        .route("/agg_sig_and_broadcast", post(agg_sig_and_broadcast));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}