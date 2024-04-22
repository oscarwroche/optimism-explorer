use std::sync::Arc;

use application::queries::get_latest_block::get_latest_block_query;
use axum::{routing::get, Router};
use infrastructure::adapters::op_stack_adapter::OpStackAdapter;

pub mod application;
pub mod domain;
pub mod infrastructure;

const RPC_URL: &str = "http://localhost:8545";

#[tokio::main]
async fn main() {
    let blockchain_polling_service = Arc::new(OpStackAdapter::new(RPC_URL));
    let app = Router::new().route(
        "/block",
        get({
            let blockchain_polling_service = Arc::clone(&blockchain_polling_service);
            move || get_latest_block_query(blockchain_polling_service)
        }),
    );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
