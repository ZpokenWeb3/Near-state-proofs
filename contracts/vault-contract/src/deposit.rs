use crate::*;

impl VaultContract {
    pub fn deposit_from_ft(
        &mut self,
        token_amount: WBalance,
        sender_id: AccountId,
        reciever_addr: AccountId,
        asset_id: AccountId,
    ) -> PromiseOrValue<WBalance> {
        self.count_param += 1;

        self.receiver_addr = reciever_addr.clone();
        self.deposited_amount = token_amount.0.clone();
        self.asset_id = asset_id.clone();

        self.bridge_info.insert(
            &sender_id,
            &BridgeInfo {
                receiver_addr: reciever_addr,
                asset_id,
                deposited_amount: token_amount.0,
            },
        );

        PromiseOrValue::Value(U128::from(0))
    }
}

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
