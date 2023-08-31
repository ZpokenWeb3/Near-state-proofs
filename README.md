## Message Proof of Inclusion for Near Blockchain

#### Let's take a look at a factory.bridge.near contract, that are involved in a Near-Ethereum bridge functioning

The algorithm for proof of inclusion the following:

1) get storage key-values for the contract and respective proof for that through RPC command

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

2) having result that can be deserialized into ViewStateResult

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
    "block_hash": "Hpq9PKE4oS1QurBVVaG6FzQx1AqPjYrn85jAqvmV1ngY",
    "block_height": 99917403,
    "proof": [
      "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"
    ],
    "values": [
      {
        "key": "U1RBVEU=",
        "value": "FwAAAGNsaWVudC1ldGgyLmJyaWRnZS5uZWFyAAAAAAAAAAAAAAAAAAAAAA=="
      }
    ]
  }
}
```

3) have to ensure that the proof is valid by checking key-value existence through near-core verification logic. Proof
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

4) we will iterate through all the key-value pairs in the state and verify that the proof is valid for them

And success - our result is matching the outcome root, so it means that our receipt was indeed processed by the
blockchain.









