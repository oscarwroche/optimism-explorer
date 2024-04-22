use std::sync::Arc;

use axum::{response::IntoResponse, Extension};

use crate::domain::services::blockchain_polling_service::BlockchainPollingService;

pub async fn get_latest_block_query(
    Extension(blockchain_polling_service): Extension<Arc<dyn BlockchainPollingService>>,
) -> impl IntoResponse {
    blockchain_polling_service.get_latest_block().await
}
