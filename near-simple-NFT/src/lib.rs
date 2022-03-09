use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{near_bindgen, AccountId};
near_sdk::setup_alloc!();

/**
 * We’ll take a moment to understand why we need to do this in Near. Why could we not directly create a variable owners why did we have to put it inside a struct?
 * The answer to this lies in the fact that, on near there are accounts.
 * Each account can consist of the following things.
    * The Account ID
    * Balance in Near Tokens
    * At most 1 contract
    * Storage
 * When we will be deploying a contract, we will be creating an account and putting the contract inside that account.
 * Every contract being a part of an account also has the native support to handle and store money.
 * The storage is independent of the contract space. So, we need to make sure that the contract stores the information in a way that is consistent and usable even outside of the contract. Remember all the data on the blockchain is public information.
 * So, to make it storable in the storage part of the account, we need to create a struct that is serializable. The struct is stored in a serializable way. So when we want to alter the owners variable the near-runtime will pull the data from the storage space, deserialize it, make the modifications, serialize it and store it back in the storage.
 */

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]

pub struct NftOwners {
    owners: UnorderedMap<String, AccountId>,
}

// default is like constructor in a class.
impl Default for NftOwners {
    fn default() -> Self {
        Self {
            // When we create a new UnorderedMap we also need to provide an identifier ”o” which will identify this map on the storage space.
            owners: UnorderedMap::new(b"o"),
        }
    }
}

#[near_bindgen]
impl NftOwners {
    pub fn set_owner(&mut self, token_id: String, account_id: AccountId) {
        self.owners.insert(&token_id, &account_id);
    }

    pub fn get_owner(&self, token_id: String) -> AccountId {
        match self.owners.get(&token_id) {
            Some(owner) => owner,
            None => "No owner found".to_string(),
        }
    }
}


// testing contract
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};
    // mockup blockchain to test smart contract
    fn get_context(predecessor_account_id: String, storage_usage: u64) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "jane.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id,
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn set_owner() {
        let context = get_context("you.testnet".to_string(), 0);
        testing_env!(context);
        let mut contract = NftOwners::default();
        let my_token = "my_token".to_string();
        let owner_account_id = "you.testnet".to_string();
        contract.set_owner(my_token.clone(), owner_account_id.clone());
        let owner_of_nft = contract.get_owner(my_token);
        assert_eq!(owner_of_nft, owner_account_id);
    }
}
