use crate::*;

#[near_bindgen]
impl VaultContract {
    pub fn view_receiver_addr(&self) -> String {
        self.receiver_addr.clone().to_string()
    }

    pub fn view_asset_id(&self) -> String {
        self.asset_id.clone().to_string()
    }

    pub fn view_deposited_amount(&self) -> WBalance {
        self.deposited_amount.into()
    }

    pub fn view_count(&self) -> u128 {
        self.count_param.clone()
    }

    pub fn view_depositor_info(&self, depositor_addr: AccountId) -> BridgeInfo {
        let depositor_bridge_info = self.bridge_info.get(&depositor_addr).unwrap_or_else(|| {
            panic!("Bridge info for account: {depositor_addr} not found");
        });

        BridgeInfo {
            receiver_addr: depositor_bridge_info.receiver_addr,
            asset_id: depositor_bridge_info.asset_id,
            deposited_amount: depositor_bridge_info.deposited_amount,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_view_single_slots() {
        let contract = VaultContract::initialize_vault_contract(12);

        let asset_id = contract.view_asset_id();
        let receiver_addr = contract.view_receiver_addr();
        let deposited_amount = contract.view_deposited_amount();

        dbg!(asset_id);
        dbg!(receiver_addr);
        dbg!(deposited_amount);
    }
}
