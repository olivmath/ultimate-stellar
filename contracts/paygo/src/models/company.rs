use crate::models::employee::Employee;
use soroban_sdk::{Address, String, Vec};

#[derive(Clone)]
pub struct Company {
    pub name: String,
    pub description: String,
    pub employees: Vec<Employee>,
    pub owner: Address,
    pub balance: i128,
    pub is_active: bool,
}
