mod utils;
mod proof_verifier;
mod raw_node;
mod nibble_slice;

use std::str::FromStr;
use near_primitives::types::AccountId;
use reqwest::Client;
use crate::proof_verifier::ProofVerifier;
use crate::utils::{Config, ViewStateParams, ViewStateRequest, ViewStateResponseForProof};


use std::fs;


#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // reading config for the account
    let config_str = fs::read_to_string("./config.json").unwrap();

    let config: Config = serde_json::from_str(&config_str).unwrap();

    let account_id = AccountId::from_str(&*config.account).unwrap();

    // querying state for the account
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


    // constructing and verifying proof for all key-value pairs
    let view_state_response_for_proof: ViewStateResponseForProof = client.post("https://rpc.mainnet.near.org")
        .json(&view_state_request)
        .send()
        .await?
        .json()
        .await?;

    let proof_verifier = ProofVerifier::new(view_state_response_for_proof.result.proof).unwrap();

    assert!(!proof_verifier.get_nodes().is_empty(), "Proof isn't valid");
    let mut result_proof_boolean = vec![];

    for root in proof_verifier.get_nodes_hashes() {
        for state_item in &view_state_response_for_proof.result.values {
            let is_true = proof_verifier.verify(&root, &account_id, &state_item.key.to_vec(), Some(&state_item.value.to_vec()));
            if is_true {
                result_proof_boolean.push((is_true, root));
            }
        }
    }
    assert_eq!(result_proof_boolean.len(), view_state_response_for_proof.result.values.len(), "Proof for the key-value pair isn't verified.");

    assert!(result_proof_boolean.iter().any(|(is_true, _)| *is_true == true), "Proof for the key-value pair isn't verified.");

    Ok(())
}