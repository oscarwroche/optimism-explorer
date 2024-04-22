use serde::Serialize;

#[derive(Serialize)]
pub struct Block {
    pub hash: String,
}
