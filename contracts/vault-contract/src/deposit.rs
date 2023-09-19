use crate::*;

impl VaultContract {
    pub fn deposit(
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
