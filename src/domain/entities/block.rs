use serde::Serialize;

type BlockHash = String;
type TransactionHash = String;
type BlockNumber = u64;

#[derive(Serialize)]
pub struct Block {
    pub hash: BlockHash,
    pub number: BlockNumber,
    pub transaction_hashes: Vec<TransactionHash>,
}
