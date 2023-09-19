mod nibble_slice;
mod proof_verifier;
mod raw_node;
mod utils;

use crate::proof_verifier::ProofVerifier;
use crate::utils::{Config, ViewStateParams, ViewStateRequest, ViewStateResponseForProof};
use near_primitives::types::AccountId;
use reqwest::{Client, Error};
use std::str::FromStr;
use std::fs::File;
use std::io::Write;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // open outcome for writing
    let mut file = File::create("key_value_with_proofs.txt").expect("Unable to create file");

    // reading config for the account
    let config_str = fs::read_to_string("config.json").unwrap();

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
    if client
        .post("https://rpc.mainnet.near.org")
        .json(&view_state_request)
        .send()
        .await?.json::<ViewStateResponseForProof>().await.is_err() {
        panic!(
            "State of contract {}  is too large to be viewed",
            account_id
        );
    } else {
        let view_state_response_for_proof: ViewStateResponseForProof = client
            .post("https://rpc.mainnet.near.org")
            .json(&view_state_request)
            .send()
            .await?
            .json()
            .await?;

        let proof_verifier =
            ProofVerifier::new(view_state_response_for_proof.result.proof).unwrap();

        assert!(!proof_verifier.get_nodes().is_empty(), "Proof isn't valid");
        let mut result_proof_boolean = vec![];

        for root in proof_verifier.get_nodes_hashes() {
            for state_item in &view_state_response_for_proof.result.values {
                let is_true = proof_verifier.verify(
                    &root,
                    &account_id,
                    &state_item.key.to_vec(),
                    Some(&state_item.value.to_vec()),
                );
                if is_true {
                    result_proof_boolean.push((is_true, root));

                    writeln!(file, "Key: {:?}", state_item.key).expect("Unable to write to file");
                    writeln!(file, "Value: {:?}", state_item.value).expect("Unable to write to file");
                    writeln!(file, "Proof: {:?}", root).expect("Unable to write to file");
                    writeln!(file, "-----------------------------").expect("Unable to write to file");
                }
            }
        }

        assert_eq!(
            result_proof_boolean.len(),
            view_state_response_for_proof.result.values.len(),
            "Proof for the key-value pair isn't verified."
        );

        assert!(
            result_proof_boolean
                .iter()
                .any(|(is_true, _)| *is_true == true),
            "Proof for the key-value pair isn't verified."
        );

        Ok(())
    }
}
