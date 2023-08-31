use near_primitives::views::{ViewStateResult};
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
pub struct ViewStateResponseForProof {
    pub result: ViewStateResult,
}


/// Item of the state, key and value are serialized in base64 and proof for inclusion of given state item.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct StateItemValues {
    pub key: String,
    pub value: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Chunk {
    chunk_hash: String,
    encoded_merkle_root: String,
    outcome_root: String,
    outgoing_receipts_root: String,
    prev_block_hash: String,
    prev_state_root: String,
    rent_paid: String,
    shard_id: u64,
    signature: String,
    tx_root: String,
    validator_proposals: Vec<String>,
    // Assuming it's a Vec<String> but might need to adjust based on actual data
    validator_reward: String,
}

