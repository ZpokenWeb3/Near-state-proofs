message proof of inclusion for Near Blockchain

Due to the contract state size limit, we are using less robust contracts involved in factory.bridge.near work

view state for the mainnet account through command:

```
http post https://rpc.mainnet.near.org jsonrpc=2.0 id=dontcare method=query \
params:='{
"request_type": "view_state",
"finality": "final",
"account_id": "prover.bridge.near",
"prefix_base64": "",
"include_proof": true
}'
```

obtaining block_hash and using it in the next step for the validation 


```
http post https://rpc.mainnet.near.org jsonrpc=2.0 id=dontcare method=block \
params:='{
"block_id": "3uEiG6KZocinAgftKHGevdexXDD4PD1GY8vFKfKeoU4e"
}'
```






