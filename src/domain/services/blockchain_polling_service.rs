use axum::{async_trait, Json};

use crate::domain::{entities::block::Block, errors::error::AppError};

#[async_trait]
pub trait BlockchainPollingService: Sync + Send {
    async fn get_latest_block(&self) -> Result<Json<Block>, AppError>;
}
