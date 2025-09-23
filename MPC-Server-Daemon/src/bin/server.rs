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

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/generate", post(generate_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}