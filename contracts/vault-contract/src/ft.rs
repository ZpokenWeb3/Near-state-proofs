use crate::*;

use crate::utils::Actions;
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::serde_json;
use near_sdk::AccountId;

#[near_bindgen]
impl FungibleTokenReceiver for VaultContract {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        assert!(
            Balance::from(amount) > 0,
            "Amount should be a positive number"
        );

        let asset_id = env::predecessor_account_id();

        log!(format!("sender_id {sender_id}, msg {msg}"));

        let action: Actions = serde_json::from_str(&msg).expect("Incorrect command in transfer");
        let receiver_addr = sender_id.clone();
        match action {
            Actions::Deposit {} => self.deposit_from_ft(amount, sender_id, receiver_addr, asset_id),
        }
    }
}
