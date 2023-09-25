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

#[derive(Debug, Serialize, Deserialize)]
pub struct ViewStateResponseForValues {
    pub result: ViewStateResultForValues,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct ViewStateResultForValues {
    pub values: Vec<StateItemForValue>,
    pub proof: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct StateItemForValue {
    pub key: String,
    pub value: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ResultData {
    pub block_height: u128,
}


/// Item of the state, key and value are serialized in base64 and proof for inclusion of given state item.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct StateItemValues {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub account: String,
    pub network: u8,
}


pub fn decode_base64(encoded: &str) -> Result<String, &'static str> {
    // Decode the base64 string to bytes
    let decoded_bytes = match base64::decode(encoded) {
        Ok(bytes) => bytes,
        Err(_) => return Err("Invalid base64 encoding"),
    };

    // Convert the bytes to a UTF-8 string
    match String::from_utf8(decoded_bytes) {
        Ok(decoded_string) => Ok(decoded_string),
        Err(_) => Err("Invalid UTF-8 sequence"),
    }
}

