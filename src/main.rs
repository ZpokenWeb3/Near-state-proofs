mod utils;
mod proof_verifier;
mod raw_node;
mod nibble_slice;

use reqwest::Client;
use crate::utils::{BlockParams, BlockRequest, BlockResponse, ViewStateParams, ViewStateRequest, ViewStateResponse};

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

    // TODO verify by prove verifier that proof is valid by constructing all the way to the expected root from the .verify function

    Ok(())
}
