## Message Proof of Inclusion for Near Blockchain

#### Let's take a look at the contract [first.zpoken-vault-contract.testnet](https://testnet.nearblocks.io/address/first.zpoken-vault-contract.testnet), which stores the desired information related to the bridge functioning

The structure of the contract storage is the following

```rust
   #[near_bindgen]
   #[derive(BorshDeserialize, BorshSerialize)]
   pub struct VaultContract {
       // depositor_addr -> BridgeInfo
       bridge_info: UnorderedMap<AccountId, BridgeInfo>,
   
       receiver_addr: AccountId,
       asset_id: AccountId,
       deposited_amount: Balance,
   
       count_param: Balance,
   }
   
   #[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
   #[serde(crate = "near_sdk::serde")]
   pub struct BridgeInfo {
       pub receiver_addr: AccountId,
       pub asset_id: AccountId,
       pub deposited_amount: Balance,
   }
```

and the respective deposit function

```rust
   #[near_bindgen]
   impl VaultContract {
       pub fn deposit(
           &mut self,
           receiver_addr: AccountId,
           asset_id: AccountId,
           token_amount: WBalance,
       ) -> PromiseOrValue<WBalance> {
           self.count_param += 1;
   
           self.receiver_addr = receiver_addr.clone();
           self.deposited_amount = token_amount.0.clone();
           self.asset_id = asset_id.clone();
   
           let sender_id = env::signer_account_id();
   
           self.bridge_info.insert(
               &sender_id,
               &BridgeInfo {
                   receiver_addr,
                   asset_id,
                   deposited_amount: token_amount.0,
               },
           );
   
           PromiseOrValue::Value(U128::from(0))
       }
   }

```

The inner algorithm for proof of inclusion the following:

1) Configure account in config.json file 
   
```json
   {
   "account": "first.zpoken-vault-contract.testnet",
   "network": 0 // 0 - for TESTNET, 1 for MAINNET
   }
```

2) Make a sample deposit transaction https://testnet.nearblocks.io/txns/2zF2wNDprwBSDV5bB9QwMxL6EF573X3gXvUqFL999Me7  to fill up the desired storage slots with an information we want to proove.


3) Get storage key-values for the contract and respective proof for that through RPC command

```
   http post https://rpc.testnet.near.org jsonrpc=2.0 id=dontcare method=query \
   params:='{
   "request_type": "view_state",
   "finality": "final",
   "account_id": "first.zpoken-vault-contract.testnet",
   "prefix_base64": "",
   "include_proof": true
   }'
```

4) Having result that can be deserialized into ViewStateResult (note that this is an example call, latest query might be
   different due to rapid block production in Near)

```rust
   #[serde_as]
   #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Clone)]
   pub struct ViewStateResult {
       pub values: Vec<StateItem>,
       #[serde_as(as = "Vec<Base64>")]
       #[serde(default, skip_serializing_if = "Vec::is_empty")]
       pub proof: Vec<Arc<[u8]>>,
   }
   
   /// Item of the state, key and value are serialized in base64 and proof for inclusion of given state item.
   #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Clone)]
   pub struct StateItem {
       pub key: StoreKey,
       pub value: StoreValue,
   }
```

```json
   {
     "id": "dontcare",
     "jsonrpc": "2.0",
     "result": {
       "block_hash": "6SfZ8ynqHvTLbCpe8HTADBDiFzaANzUGEoxB4mVwckaG",
       "block_height": 139251814,
       "proof": [
         "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",
         "AcAAlItbZsiaJKhUMZ3LsPjhkNAONA+ENlHP/CN19EazmFnkcNwLgHFFLaXLewuPXeX+XpzzwippQIcUCOC8+nraSLQAAgAAAAAA",
         "ARQEGB7RURXKQ6MYk9eTac2iTZz+p8wxi3IUeFz/Z5UJrQ6YDDq1cBqSTVGqyg8fBtUvYvzb4NCrdnFZbdW+TroBbqFpr6ZRZuNgPNsy9DsbfdztJPbW58Y9nUSVWoXeMG+CrgwAAAAAAAA=",
         "AwEAAAAXTRufkWS9fPKH43umF+gsReLQmfKx/MSZVY5oURqMPxE6CwAAAAAAAA==",
         "AQECrtOEJsXatQg+kBPeJELH7AHydfXit3MjRR2pCINkSifcmmFTODle4sx+jC8w/dY1cL47BiOcq9wb+1hyL/ZF4wYLAAAAAAAA",
         "Ax0AAAAAb2tlbi12YXVsdC1jb250cmFjdC50ZXN0bmV0LMEsSo5VdOMDoRIrW+rTuBKiOaZ5Rqx/9IfTPLhUGr+s8QMAAAAAAAA=",
         "ASEAygAC6mpmdfFKltqPe6339WCdYp77wndhOfPuMplU+ek582ItXXkM4ru4TydWALGMMpCxP04lZHwMC2LwrdQxPYUDAAAAAAAA",
         "AwEAAAAQRIkLJK86YIFEEf5/TD6WiyAhdPyIOhHx55/tXpTuMZmEAgAAAAAAAA==",
         "AcAAPQIm6ZDCBs90KBF1j8XAg1NtDCDp2r4RtWQD50DoWJ3r76bmB3dsie8njZtD8C+UYKK68+9kiztiibSNua7DplACAAAAAAAA",
         "AQAKDhyb6cygzcVCnsIMZUUzmP59vtteJl9AKVTbzUeqEvAdIPcxtEQz1rowShJEW5yMHoyOYSrM+hxflTQfAhTfVnkBAAAAAAAA",
         "ACIAAAAgHQAAAHpwb2tlbi12YXVsdC1jb250cmFjdC50ZXN0bmV0CAAAAK9VcPWhgQt694yvS8cKZg8N9R5CuvkdTeWyMo3g6D38sAAAAAAAAAA=",
         "AAkAAAAgAAAAAAAAAAAhAAAABwpyiO2NKg0Eg13ADCes3FNsfeIYnWRc4Y0tHkgZSZyXAAAAAAAAAA==",
         "AAkAAAA2AAAAAAAAAAAvAAAAkttDOGM7NC9FgS7XfyqJ8tk4VzlP63vfNWjnPzOIgJSlAAAAAAAAAA==",
         "AAUAAAAzVEFURWEAAAAycNSx8ssO01dJc60gSQF3xNkCQnFd3dbTLijwjXFyxs8AAAAAAAAA"
       ],
       "values": [
         {
           "key": "AGkdAAAAenBva2VuLXZhdWx0LWNvbnRyYWN0LnRlc3RuZXQ=",
           "value": "AAAAAAAAAAA="
         },
         {
           "key": "AGsAAAAAAAAAAA==",
           "value": "HQAAAHpwb2tlbi12YXVsdC1jb250cmFjdC50ZXN0bmV0"
         },
         {
           "key": "AHYAAAAAAAAAAA==",
           "value": "DQAAAHJlY2VpdmVyLm5lYXIKAAAAYXNzZXQubmVhcgAAANB5AKE/eVx2BgAAAAA="
         },
         {
           "key": "U1RBVEU=",
           "value": "AgAAAABpAQAAAAAAAAACAAAAAGsBAAAAAAAAAAIAAAAAdg0AAAByZWNlaXZlci5uZWFyCgAAAGFzc2V0Lm5lYXIAAADQeQChP3lcdgYAAAAAAwAAAAAAAAAAAAAAAAAAAA=="
         }
       ]
     }
   }
```

5) Have to ensure that the proof is valid by checking key-value existence through near-core verification logic. Proof
   itself is all the nodes visited, that store the different pieces of a contract's / account's metadata. near-core
   verification logic is the following

```rust
   pub(crate) fn verify(
       &self,
       state_root: &StateRoot,
       account_id: &AccountId,
       key: &[u8],
       expected: Option<&[u8]>,
   ) -> bool {
       let query = trie_key_parsers::get_raw_prefix_for_contract_data(account_id, key);
       let mut key = NibbleSlice::new(&query);
   
       let mut expected_hash = state_root;
       while let Some(node) = self.nodes.get(expected_hash) {
           match &node.node {
               RawTrieNode::Leaf(node_key, value) => {
                   let nib = &NibbleSlice::from_encoded(&node_key).0;
                   return if &key != nib {
                       expected.is_none()
                   } else {
                       expected.map_or(false, |expected| value == expected)
                   };
               }
               RawTrieNode::Extension(node_key, child_hash) => {
                   expected_hash = child_hash;
   
                   // To avoid unnecessary copy
                   let nib = NibbleSlice::from_encoded(&node_key).0;
                   if !key.starts_with(&nib) {
                       return expected.is_none();
                   }
                   key = key.mid(nib.len());
               }
               RawTrieNode::BranchNoValue(children) => {
                   if key.is_empty() {
                       return expected.is_none();
                   }
                   match children[key.at(0)] {
                       Some(ref child_hash) => {
                           key = key.mid(1);
                           expected_hash = child_hash;
                       }
                       None => return expected.is_none(),
                   }
               }
               RawTrieNode::BranchWithValue(value, children) => {
                   if key.is_empty() {
                       return expected.map_or(false, |exp| value == exp);
                   }
                   match children[key.at(0)] {
                       Some(ref child_hash) => {
                           key = key.mid(1);
                           expected_hash = child_hash;
                       }
                       None => return expected.is_none(),
                   }
               }
           }
       }
       false
   }
```

6) We will iterate through all the key-value pairs in the state and verify that the proof is valid for them and check
   that there if a respective amount of proofs for each key-value


7) And success, so it means that our value was indeed included by the blockchain.


8) 







