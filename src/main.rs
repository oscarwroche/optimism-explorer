use ethers::prelude::*;

const RPC_URL: &str = "http://localhost:8545";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Http>::try_from(RPC_URL)?;
    let block_number: U64 = provider.get_block_number().await?;
    let block = provider.get_block(block_number).await?;
    let transaction = provider
        .get_transaction(block.clone().unwrap().transactions[0])
        .await?;
    println!("{block_number}");
    println!("Got block: {}", serde_json::to_string(&block)?);
    println!("Got transaction: {}", serde_json::to_string(&transaction)?);

    Ok(())
}
