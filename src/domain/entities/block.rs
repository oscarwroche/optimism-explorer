use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockHash(pub String);

#[derive(Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockNumber(pub u64);

#[derive(Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TransactionHash(pub String);

#[derive(Serialize, Debug)]
pub struct Block {
    pub hash: BlockHash,
    pub number: BlockNumber,
    pub transaction_hashes: Vec<TransactionHash>,
}
