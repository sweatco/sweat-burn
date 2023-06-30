use near_sdk::env;

use crate::Contract;

impl Contract {
    pub(crate) fn assert_authorized(&self) {
        let predecessor = env::predecessor_account_id();
        assert!(
            predecessor == env::current_account_id()
                || self.authorized_accounts.contains(&predecessor),
            "Unathorized"
        );
    }
}
