
echo y | near delete first.zpoken-vault-contract.testnet zpoken-vault-contract.testnet

wait

near create-account first.zpoken-vault-contract.testnet --masterAccount zpoken-vault-contract.testnet

wait

near deploy first.zpoken-vault-contract.testnet \
  --wasmFile target/wasm32-unknown-unknown/release/vault_contract.wasm \
  --initFunction 'initialize_vault_contract' \
  --initArgs '{
            "count_param": 1
        }'

wait

near view first.zpoken-vault-contract.testnet view_count '{}'

wait

near call first.zpoken-vault-contract.testnet deposit '{"receiver_addr":"receiver.near", "asset_id": "asset.near", "token_amount": "2000000000000000000000000000"}' --gas 300000000000000 --accountId zpoken-vault-contract.testnet

wait

near view first.zpoken-vault-contract.testnet view_count '{}'
near view first.zpoken-vault-contract.testnet view_asset_id '{}'
near view first.zpoken-vault-contract.testnet view_receiver_addr '{}'
near view first.zpoken-vault-contract.testnet view_deposited_amount '{}'
near view first.zpoken-vault-contract.testnet view_sender '{}'
