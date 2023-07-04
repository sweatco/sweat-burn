use near_sdk::env;

use crate::Contract;

impl Contract {
    pub(crate) fn assert_authorized(&self) {
        assert!(
            env::predecessor_account_id() == self.manager_account_id,
            "Unathorized"
        );
    }
}
