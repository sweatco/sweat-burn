use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedSet;
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
    authorized_accounts: UnorderedSet<AccountId>,
}

#[ext_contract(ext_self)]
trait SelfCallbacks {
    fn after_burn(&self, amount: U128) -> U128;
}

pub trait Authorization {
    fn add_authorized_account(&mut self, account_id: AccountId);

    fn remove_authorized_account(&mut self, account_id: AccountId);

    fn get_authorized_accounts(&self) -> Vec<AccountId>;
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(token_account_id: AccountId) -> Self {
        Self {
            token_account_id,
            authorized_accounts: UnorderedSet::new(b"a"),
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
impl Authorization for Contract {
    fn add_authorized_account(&mut self, account_id: AccountId) {
        self.assert_authorized();
    
        if !self.get_authorized_accounts().contains(&account_id) {
            self.authorized_accounts.insert(&account_id);
        }
    }

    fn remove_authorized_account(&mut self, account_id: AccountId) {
        self.assert_authorized();

        self.authorized_accounts.remove(&account_id);
    }

    fn get_authorized_accounts(&self) -> Vec<AccountId> {
        let mut result = self.authorized_accounts.to_vec();
        result.push(env::current_account_id());

        result
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
