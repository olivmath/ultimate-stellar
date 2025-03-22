use soroban_sdk::{Address, String};

#[derive(Clone, Debug)]
pub struct Employee {
    pub name: String,
    pub account_id: Address,
    pub budget: i128,
    pub last_payment: u64,
}