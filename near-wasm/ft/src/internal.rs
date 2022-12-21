use near_sdk::require;

use crate::*;

impl Contract {
    /// Internal method for depositing some amount of FTs into an account.
    pub(crate) fn internal_deposit(&mut self, account_id: &AccountId, amount: Balance) {
        // Get the current balance of the account.
        // let balance = self.accounts.get(&account_id).unwrap_or(0);
        let balance = self.internal_unwrap_balance_of(account_id);

        // Add the amount to the balance and insert the new balance into the accounts map
        if let Some(new_balance) = balance.checked_add(amount) {
            self.accounts.insert(account_id, &new_balance);
        } else {
            env::panic_str("Balance overflow");
        }
    }

    /// Internal method for measuring how many bytes it takes to insert the longest possible account ID into our map
    /// This will insert the account, measure the storage, and remove the account. It is called in the initialization function.
    pub(crate) fn measure_bytes_for_longest_account_id(&mut self) {
        let initial_storage_usage = env::storage_usage();
        let tmp_account_id = AccountId::new_unchecked("a".repeat(64));
        self.accounts.insert(&tmp_account_id, &0u128);
        self.bytes_for_longest_account_id = env::storage_usage() - initial_storage_usage;
        self.accounts.remove(&tmp_account_id);
    }

    /// Internal method for registering an account with the contract.
    pub(crate) fn internal_register_account(&mut self, account_id: &AccountId) {
        if self.accounts.insert(account_id, &0).is_some() {
            env::panic_str("The account is already registered");
        }
    }

    pub(crate) fn internal_unwrap_balance_of(&self, account_id: &AccountId) -> Balance {
        match self.accounts.get(account_id) {
            Some(balance) => balance,
            None => {
                env::panic_str(format!("The account {} is not registered", &account_id).as_str())
            }
        }
    }
}
