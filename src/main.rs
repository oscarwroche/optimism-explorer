use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use ethers::{core::abi::AbiEncode, prelude::*};
use serde::Serialize;

const RPC_URL: &str = "http://localhost:8545";

#[tokio::main]
async fn main() {
    let app = Router::new().route("/transactions", get(get_blocks));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct Block {
    hash: String,
}

// Make our own error that wraps `anyhow::Error`.
struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

async fn get_blocks() -> Result<Json<Block>, AppError> {
    let provider = Provider::<Http>::try_from(RPC_URL)?;
    let block_number: U64 = provider.get_block_number().await?;
    let block = provider.get_block(block_number).await?;
    // let transaction = provider
    //     .get_transaction(block.clone().unwrap().transactions[0])
    //     .await?;

    println!("Got block: {}", serde_json::to_string(&block)?);
    //    println!("Got transaction: {}", serde_json::to_string(&transaction)?);
    // insert your application logic here

    let block1 = Block {
        hash: block.unwrap().hash.unwrap().encode_hex(),
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    Ok(Json(block1))
}
