use std::sync::Arc;
use near_primitives::hash::CryptoHash;
use near_primitives::types::{StoreKey, StoreValue};
use near_primitives::views::{StateItem, ViewStateResult};
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


#[derive(Debug, Serialize, Deserialize)]
pub struct ViewStateResponseForValues {
    pub result: ResultData,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ResultData {
    pub block_hash: CryptoHash,
    pub values: Vec<StateItemValues>,
    pub proof: Vec<String>,
}

/// Item of the state, key and value are serialized in base64 and proof for inclusion of given state item.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct StateItemValues {
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
    pub header: BlockHeader,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockHeader {
    pub block_merkle_root: String,
    pub chunk_headers_root: String,
    pub prev_state_root: String,
}

