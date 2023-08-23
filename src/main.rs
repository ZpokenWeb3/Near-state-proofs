use reqwest;
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use near_primitives::serialize::to_base64;


#[derive(Debug, Serialize)]
struct ViewStateRequest {
    jsonrpc: &'static str,
    id: &'static str,
    method: &'static str,
    params: ViewStateParams,
}

#[derive(Debug, Serialize)]
struct ViewStateParams {
    request_type: &'static str,
    finality: &'static str,
    account_id: String,
    prefix_base64: &'static str,
    include_proof: bool,
}


#[derive(Debug, Serialize, Deserialize)]
struct ViewStateResponse {
    result: ResultData,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResultData {
    block_hash: String,
    block_height: u64,
    proof: Vec<String>,
    values: Vec<KeyValue>,
}

#[derive(Debug, Serialize, Deserialize)]
struct KeyValue {
    key: String,
    value: String,
}

#[derive(Debug, Serialize)]
struct BlockRequest {
    jsonrpc: &'static str,
    id: &'static str,
    method: &'static str,
    params: BlockParams,
}

#[derive(Debug, Serialize)]
struct BlockParams {
    block_id: String,
}


#[derive(Debug, Serialize, Deserialize)]
struct BlockResponse {
    result: BlockResultData,
}

#[derive(Debug, Serialize, Deserialize)]
struct BlockResultData {
    author: String,
    // chunks: Vec<Chunk>,
    header: BlockHeader,
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
struct BlockHeader {
    // approvals: Vec<Option<String>>,
    block_merkle_root: String,
    chunk_headers_root: String,
    prev_state_root: String,
    // chunks_included: u128,
}


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let account_id = "prover.bridge.near".to_string();

    // Query the state of the smart contract
    let view_state_request = ViewStateRequest {
        jsonrpc: "2.0",
        id: "dontcare",
        method: "query",
        params: ViewStateParams {
            request_type: "view_state",
            finality: "final",
            account_id,
            prefix_base64: "",
            include_proof: true,
        },
    };

    let client = Client::new();

    let view_state_response: ViewStateResponse = client.post("https://rpc.mainnet.near.org")
        .json(&view_state_request)
        .send()
        .await?
        .json()
        .await?;

    println!("result {:?}", view_state_response.result);

    let block_request = BlockRequest {
        jsonrpc: "2.0",
        id: "dontcare",
        method: "block",
        params: BlockParams {
            block_id: view_state_response.result.block_hash,
        },
    };

    let block_response: BlockResponse = client.post("https://rpc.mainnet.near.org")
        .json(&block_request)
        .send()
        .await?.json().await?
        ;

    println!("result {:?}", block_response.result);


    Ok(())
}
