#![allow(dead_code)]
use std::collections::HashMap;

// Define the write model
pub struct BankAccount {
    id: u32,
    balance: i32,
}

pub struct AccountBalance {
    id: u32,
    balance: i32,
}

pub struct QueryService {
    accounts: HashMap<u32, i32>,
}

pub struct CommandService {
    accounts: HashMap<u32, BankAccount>,
}

impl BankAccount {
    fn new(id: u32) -> Self {
        BankAccount { id, balance: 0 }
    }

    fn deposit(&mut self, amount: i32) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: i32) {
        self.balance -= amount;
    }
}

impl QueryService {
    fn new() -> Self {
        QueryService {
            accounts: HashMap::new(),
        }
    }

    fn get_balance(&self, id: u32) -> Option<i32> {
        self.accounts.get(&id).copied()
    }

    fn update_balance(&mut self, balance: AccountBalance) {
        self.accounts.insert(balance.id, balance.balance);
    }
}

impl CommandService {
    fn new() -> Self {
        CommandService {
            accounts: HashMap::new(),
        }
    }

    fn create_account(&mut self, id: u32) {
        self.accounts.insert(id, BankAccount::new(id));
    }

    fn deposit(&mut self, id: u32, amount: i32) {
        if let Some(account) = self.accounts.get_mut(&id) {
            account.deposit(amount);
        }
    }

    fn withdraw(&mut self, id: u32, amount: i32) {
        if let Some(account) = self.accounts.get_mut(&id) {
            account.withdraw(amount);
        }
    }

    fn get_balance(&self, id: u32) -> Option<i32> {
        self.accounts.get(&id).map(|account| account.balance)
    }

    fn update_balance(&mut self, id: u32) -> Option<AccountBalance> {
        if let Some(account) = self.accounts.get(&id) {
            Some(AccountBalance {
                id,
                balance: account.balance,
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod cqrs_tests {
    use super::{CommandService, QueryService};

    #[test]
    fn test_cqrs() {
        let mut command_service = CommandService::new();
        let mut query_service = QueryService::new();

        command_service.create_account(1);
        command_service.deposit(1, 100);
        command_service.withdraw(1, 50);
        let balance = command_service.get_balance(1).unwrap();
        assert_eq!(balance, 50);

        let account_balance = command_service.update_balance(1).unwrap();
        query_service.update_balance(account_balance);
        let balance = query_service.get_balance(1).unwrap();
        assert_eq!(balance, 50);
    }
}
