mod utils;
mod proof_verifier;
mod raw_node;
mod nibble_slice;

use std::str::FromStr;
use near_primitives::types::AccountId;
use reqwest::Client;
use crate::proof_verifier::ProofVerifier;
use crate::utils::{BlockParams, BlockRequest, BlockResponse, ViewStateParams, ViewStateRequest, ViewStateResponseForProof, ViewStateResponseForValues};
// use crate::utils::{BlockParams, BlockRequest, BlockResponse, StateItem, ViewStateParams, ViewStateRequest, ViewStateResponse};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let account_id = AccountId::from_str("prover.bridge.near").unwrap();

    // Query the state of the smart contract
    let view_state_request = ViewStateRequest {
        jsonrpc: "2.0",
        id: "dontcare",
        method: "query",
        params: ViewStateParams {
            request_type: "view_state",
            finality: "final",
            account_id: account_id.to_string(),
            prefix_base64: "",
            include_proof: true,
        },
    };

    let client = Client::new();

    let view_state_response_for_proof: ViewStateResponseForProof = client.post("https://rpc.mainnet.near.org")
        .json(&view_state_request)
        .send()
        .await?
        .json()
        .await?;

    let view_state_response_for_values: ViewStateResponseForValues = client.post("https://rpc.mainnet.near.org")
        .json(&view_state_request)
        .send()
        .await?
        .json()
        .await?;


    println!("result {:?}", view_state_response_for_proof.result);
    println!("result {:?}", view_state_response_for_values.result);

    let block_request = BlockRequest {
        jsonrpc: "2.0",
        id: "dontcare",
        method: "block",
        params: BlockParams {
            block_id: view_state_response_for_values.result.block_hash.to_string(),
        },
    };

    let block_response: BlockResponse = client.post("https://rpc.mainnet.near.org")
        .json(&block_request)
        .send()
        .await?.json().await?
        ;

    println!("result {:?}", block_response.result);

    // TODO verify by prove verifier that proof is valid by constructing all the way to the expected root from the .verify function
    //
    // let proof_verifier = ProofVerifier::new(view_state_response_for_proof.result.proof).unwrap();
    // let root = block_response.result.header.block_merkle_root;
    //
    // for state_item in view_state_response_for_proof.result.values {
    //
    //     // Proof for known (key, value) should succeed.
    //     assert!(
    //         proof_verifier.verify(&root, &account_id, state_item.key.try_into().unwrap(), Some(state_item.value.try_into().unwrap())),
    //         "proof isn't verified"
    //     );
    // }


    Ok(())
}