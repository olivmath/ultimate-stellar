use soroban_sdk::{Address, String, Vec};

#[derive(Clone, Debug)]
pub struct Employee {
    pub name: String,
    pub account_id: Address,
    pub budget: i128,
    pub last_payment: u64,
}

#[derive(Clone)]
pub struct Company {
    pub name: String,
    pub description: String,
    pub employees: Vec<Employee>,
    pub owner: Address,
    pub balance: i128,
    pub is_active: bool,
}
