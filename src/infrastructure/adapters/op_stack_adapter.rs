use crate::domain::{
    entities::block::Block, errors::error::AppError,
    services::blockchain_polling_service::BlockchainPollingService,
};
use axum::{
    async_trait,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use ethers::{core::abi::AbiEncode, prelude::*};

pub struct OpStackAdapter {
    provider: Provider<Http>,
}

impl OpStackAdapter {
    pub fn new(rpc_url: &str) -> Self {
        let provider = Provider::<Http>::try_from(rpc_url).unwrap();
        Self { provider }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

#[async_trait]
impl BlockchainPollingService for OpStackAdapter {
    async fn get_latest_block(&self) -> Result<Json<Block>, AppError> {
        let block_number: U64 = self.provider.get_block_number().await?;
        let latest_block = self.provider.get_block(block_number).await?;
        // let transaction = provider
        //     .get_transaction(block.clone().unwrap().transactions[0])
        //     .await?;

        println!("Got block: {}", serde_json::to_string(&latest_block)?);
        //    println!("Got transaction: {}", serde_json::to_string(&transaction)?);

        let block = Block {
            hash: latest_block.unwrap().hash.unwrap().encode_hex(),
        };

        Ok(Json(block))
    }
}
