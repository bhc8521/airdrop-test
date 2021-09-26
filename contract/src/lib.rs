
use near_contract_standards::fungible_token;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::log;
use near_sdk::serde_json::{json, from_slice};
use near_sdk::{AccountId, Balance, PromiseOrValue, PublicKey, env, near_bindgen, setup_alloc, ext_contract, log};
use near_sdk::collections::{LookupMap, UnorderedMap};
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt::Debug;
use std::hash::Hash;
use std::primitive;
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{BlockHeight, Gas, PanicOnDefault, Promise, PromiseResult};
use near_sdk::json_types::{Base58PublicKey, U128, U64, ValidAccountId};
use near_contract_standards::fungible_token::metadata::{FungibleTokenMetadata};
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;

setup_alloc!();

#[near_bindgen]
#[derive(PanicOnDefault, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Airdrop {
    private_key: String,
    nonce: u32
}

#[near_bindgen]
impl Airdrop {
    #[init]
    pub fn new() -> Self {
        Self {
            private_key: "".to_string(),
            nonce: 0
        }
    }

     /// Claim tokens for specific account that are attached to the public key this tx is signed with.
     pub fn claim(&mut self, receiver: AccountId) {
        log!("{}, {}", env::predecessor_account_id(), env::current_account_id());
        assert_eq!(
            env::predecessor_account_id(),
            env::current_account_id(),
            "Claim only can come from this account"
        );

        Promise::new(receiver).transfer(1000000000000000000000000);
        self.nonce += 1;
    }

    pub fn set_info(&mut self, private_key: String, public_key: PublicKey, nonce: u32) {
        self.private_key = private_key;
        Promise::new(env::current_account_id()).add_access_key_with_nonce(public_key, 1000000000000000000000000, env::current_account_id(), b"claim".to_vec(), nonce as u64);
    }

    pub fn get_info(&self) -> Self {
        Self {
            private_key: self.private_key.clone(),
            nonce: self.nonce
        }
    }

    // pub fn create_account_and_claim(&mut self, creator: AccountId, index: u32, receiver: AccountId, pbk: PublicKey) {
    //     assert_eq!(
    //         env::predecessor_account_id(),
    //         env::current_account_id(),
    //         "Claim only can come from this account"
    //     );
    //     let task = self.users.get_mut(&creator).unwrap().tasks.get_mut(index as usize).unwrap();
    //     assert!(task.claimed_account.get(&receiver).is_none(), "Already claimed");
    //     Promise::new(receiver.clone()).create_account().add_full_access_key(pbk);
    //     ext_fungible_token::ft_transfer(receiver.clone(), U128::from(task.amount_per_account), None, &task.token_id, 0, env::prepaid_gas() / 2);
    //     task.claimed_account.insert(receiver, task.amount_per_account);
    //     task.deposit_near -= env::prepaid_gas() as u128;
    //     task.deposit_token -= task.amount_per_account;
    // }

    // #[result_serializer(borsh)]
    // #[private]
    // pub fn on_add_token(
    //     &mut self,
    //     token_id: AccountId,
    //     #[callback_vec]
    //     #[serializer(borsh)]
    //     metadata: Result<u8, PromiseError>,
    // ) {
    //     self.tokens.insert(&token_id, &metadata);
    // }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 *
 * To run from contract directory:
 * cargo test -- --nocapture
 *
 * From project root, to run in combination with frontend tests:
 * yarn test
 *
 */
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // mock the context for testing, notice "signer_account_id" that was accessed above from env::
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn set_then_get_greeting() {
        let context = get_context(vec![], false);
        testing_env!(context);
        //let mut contract = Airdrop::new();
        //contract.add_token("dev-1632295283892-86288685865300".to_string());
    }

    #[test]
    fn get_default_greeting() {

    }
}
