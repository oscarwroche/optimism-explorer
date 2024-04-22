use std::sync::Arc;

use axum::response::IntoResponse;

use crate::domain::services::blockchain_polling_service::BlockchainPollingService;

pub async fn get_latest_block_query(
    blockchain_polling_service: Arc<dyn BlockchainPollingService>,
) -> impl IntoResponse {
    blockchain_polling_service.get_latest_block().await
}
