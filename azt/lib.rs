#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod azt {
    use ink::storage::Mapping;

    #[ink(storage)]
    #[derive(Default)]
    pub struct Azt {
        balances: Mapping<AccountId, Balance>,
        decimals: u8,
        total_supply: Balance,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
    }

    impl Azt {
        #[ink(constructor)]
        pub fn new(decimals: u8, total_supply: Balance) -> Self {
            let mut balances = Mapping::default();
            let caller = Self::env().caller();
            // The account that calls the constructor is the contrtact creator.
            // Deposit entire supply to the contract creator.
            balances.insert(caller, &total_supply);
            let total_supply = total_supply;

            Self {
                decimals,
                balances,
                total_supply,
            }
        }

        #[ink(message)]
        pub fn decimals(&self) -> u8 {
            self.decimals
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, account_id: AccountId) -> Balance {
            self.balances.get(&account_id).unwrap_or_default()
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<(), Error> {
            let from = self.env().caller();
            let from_balance = self.balance_of(from);
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }
            self.balances.insert(from, &(from_balance - value));
            let to_balance = self.balance_of(to);
            self.balances.insert(to, &(to_balance + value));
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn decimals_works() {
            // valid values
            let decimals_valid = 18;
            let azt_token = Azt::new(decimals_valid, 100_000_000);
            assert_eq!(azt_token.decimals(), decimals_valid);

            // invalid decimals greater than 18
            let decimals_invalid = 19;
            let azt_token = Azt::new(decimals_invalid, 1_000_000_000);
            assert_eq!(azt_token.decimals(), decimals_invalid);

            // negative decimals
            let decimals_negative = 1i8 as u8;
            let azt_token = Azt::new(decimals_negative, 1_000_000_000);
            assert_eq!(azt_token.decimals(), decimals_negative);
        }

        #[ink::test]
        fn total_supply_works() {
            let azt_token = Azt::new(18, 1_000_000_000);
            assert_eq!(azt_token.total_supply(), 1_000_000_000);
        }

        #[ink::test]
        fn balance_of_works() {
            let azt_token = Azt::new(18, 1_000_000_000);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            assert_eq!(azt_token.balance_of(accounts.alice), 1_000_000_000);
            assert_eq!(azt_token.balance_of(accounts.bob), 0);
        }

        #[ink::test]
        fn transfer_works() {
            let mut azt_token = Azt::new(18, 1_000_000_000);
            let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(azt_token.balance_of(accounts.bob), 0);
            assert_eq!(azt_token.transfer(accounts.bob, 10), Ok(()));
            assert_eq!(azt_token.balance_of(accounts.bob), 10);
        }
    }
}
