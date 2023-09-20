use near_sdk::serde_json::json;
use workspaces::network::Sandbox;
use workspaces::{Account, Worker};

const MOCK_TOKEN: &str = "../target/wasm32-unknown-unknown/release/mock_token.wasm";
const VAULT_CONTRACT: &str = "../target/wasm32-unknown-unknown/release/vault_contract.wasm";

pub async fn deploy_token(
    owner: &Account,
    worker: &Worker<Sandbox>,
    decimals: u8,
) -> Result<workspaces::Contract, workspaces::error::Error> {
    let wasm = std::fs::read(MOCK_TOKEN);
    let underlying = worker.dev_deploy(&wasm.unwrap()).await?;

    let _ = underlying
        .call("new_default_meta")
        .args_json(json!({ "owner_id": owner.id(),
        "name": "Wrapped Ethereum",
        "symbol": "WETH",
        "total_supply": "1000000000000000000000000000",
        "decimals": decimals
                }))
        .max_gas()
        .transact()
        .await?;

    Ok(underlying)
}

pub async fn deploy_vault_contract(
    _owner: &Account,
    worker: &Worker<Sandbox>,
) -> Result<workspaces::Contract, workspaces::error::Error> {
    let wasm = std::fs::read(VAULT_CONTRACT);
    let vault_contract = worker.dev_deploy(&wasm.unwrap()).await?;

    let _ = vault_contract
        .call("new_with_config")
        .args_json(json!({
            "count_param":  1
        }))
        .max_gas()
        .transact()
        .await?;

    Ok(vault_contract)
}
