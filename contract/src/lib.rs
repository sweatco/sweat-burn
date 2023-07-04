use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use near_sdk::{
    env, ext_contract, is_promise_success, near_bindgen, AccountId, Gas, PanicOnDefault, Promise,
};

mod asserts;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    token_account_id: AccountId,
    manager_account_id: AccountId,
}

#[ext_contract(ext_self)]
trait SelfCallbacks {
    fn after_burn(&self, amount: U128) -> U128;
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(token_account_id: AccountId, manager_account_id: AccountId,) -> Self {
        Self {
            token_account_id,
            manager_account_id,
        }
    }

    pub fn burn(&self, amount: U128) -> Promise {
        self.assert_authorized();
        
        Promise::new(self.token_account_id.clone())
            .function_call(
                "burn".into(),
                json!({ "amount": amount }).to_string().into_bytes(),
                0,
                Gas(5_000_000_000_000),
            )
            .then(
                ext_self::ext(env::current_account_id())
                    .with_static_gas(Gas(5_000_000_000_000))
                    .after_burn(amount),
            )
    }
}

#[near_bindgen]
impl SelfCallbacks for Contract {
    #[private]
    fn after_burn(&self, amount: U128) -> U128 {
        if is_promise_success() {
            amount
        } else {
            panic!("Failed on burn")
        }
    }
}
