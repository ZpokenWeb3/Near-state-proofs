use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize)]
pub struct ViewStateRequest {
    pub jsonrpc: &'static str,
    pub id: &'static str,
    pub method: &'static str,
    pub params: ViewStateParams,
}

#[derive(Debug, Serialize)]
pub struct ViewStateParams {
    pub request_type: &'static str,
    pub finality: &'static str,
    pub account_id: String,
    pub prefix_base64: &'static str,
    pub include_proof: bool,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ViewStateResponse {
    pub result: ResultData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultData {
    pub block_hash: String,
    pub block_height: u64,
    pub proof: Vec<String>,
    pub values: Vec<KeyValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize)]
pub struct BlockRequest {
    pub jsonrpc: &'static str,
    pub id: &'static str,
    pub method: &'static str,
    pub params: BlockParams,
}

#[derive(Debug, Serialize)]
pub struct BlockParams {
    pub block_id: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct BlockResponse {
    pub result: BlockResultData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockResultData {
    pub author: String,
    // chunks: Vec<Chunk>,
    pub header: BlockHeader,
}


// #[derive(Debug, Serialize, Deserialize)]
// struct Chunk {
//     chunk_hash: String,
//     encoded_merkle_root: String,
//     outcome_root: String,
//     outgoing_receipts_root: String,
//     prev_block_hash: String,
//     prev_state_root: String,
//     rent_paid: String,
//     shard_id: u64,
//     signature: String,
//     tx_root: String,
//     validator_proposals: Vec<String>,
//     // Assuming it's a Vec<String> but might need to adjust based on actual data
//     validator_reward: String,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockHeader {
    // approvals: Vec<Option<String>>,
    pub block_merkle_root: String,
    pub chunk_headers_root: String,
    pub prev_state_root: String,
    // chunks_included: u128,
}

