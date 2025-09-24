use axum::{routing::post, Router, Json};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signature::Signer;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

use solana_tss::tss;


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
    let pubkeys: Vec<Pubkey> = payload
        .keys
        .iter()
        .map(|k| Pubkey::from_str(k).expect("Invalid pubkey"))
        .collect();

        let aggkey = tss::key_agg(pubkeys, None).unwrap();
        let aggpubkey = Pubkey::new(&*aggkey.agg_public_key.to_bytes(true));

        Json(AggregateKeysResponse {
            aggregated_key: aggpubkey.to_string(),
        })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/generate", post(generate_handler))
        .route("/aggregate_keys", post(aggregate_keys_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}