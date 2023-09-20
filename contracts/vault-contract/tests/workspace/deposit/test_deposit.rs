use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use workspaces::network::Sandbox;
use workspaces::{Account, AccountId, Worker};

use crate::deposit::utils::{deploy_token, deploy_vault_contract};

const DECIMALS: u8 = 24;

async fn deposit_fixture(
    owner: &Account,
    worker: &Worker<Sandbox>,
) -> anyhow::Result<(workspaces::Contract, workspaces::Contract), anyhow::Error> {
    ////////////////////////////////////////////////////////////////////////////
    // Stage 1: Deploy contracts such as underlying, controller, and markets
    ////////////////////////////////////////////////////////////////////////////

    let mock_token = deploy_token(owner, worker, DECIMALS).await?;
    let vault_contract = deploy_vault_contract(owner, worker).await?;

    let _ = mock_token
        .call("storage_deposit")
        .args_json(json!({
            "account_id": vault_contract.id()
        }))
        .max_gas()
        .deposit(25 * 10u128.pow(23))
        .transact()
        .await?;

    let _ = mock_token
        .call("mint")
        .args_json(json!({
            "account_id": owner.id(),
            "amount": U128::from(2000000000000000000000000000)
        }))
        .max_gas()
        .transact()
        .await?;

    Ok((vault_contract, mock_token))
}

#[tokio::test]
async fn test_successful_supply() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let owner = worker.root_account()?;
    let (vault_contract, mock_token) = deposit_fixture(&owner, &worker).await?;

    let _ = owner
        .call(mock_token.id(), "ft_transfer_call")
        .args_json(json!({
            "receiver_id": vault_contract.id(),
            "amount": U128::from(1000000000000000000000000000),
             "msg": "\"Deposit\""
        }))
        .max_gas()
        .deposit(1)
        .transact()
        .await?;

    let receiver_addr: AccountId = "receiver.near".parse().unwrap();
    let asset_id: AccountId = "wrap.near".parse().unwrap();

    let _ = owner
        .call(vault_contract.id(), "deposit")
        .args_json(json!({
            "token_amount": U128::from(1000000000000000000000000000),
            "receiver_addr": receiver_addr,
            "asset_id": asset_id,
        }))
        .max_gas()
        .transact()
        .await?;

    Ok(())
}
