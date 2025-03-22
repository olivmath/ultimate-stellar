#![no_std]

mod test;

use soroban_sdk::{contract, contractimpl, vec, Env, String, Vec};

trait CRUD {}
trait Features {}

#[contract]
pub struct Company;

#[contractimpl]
impl Company {
    pub fn __constructor(e: Env) {}
}

#[contractimpl]
impl Features for Company {
    // pub fn pay_employees(e: Env) {}
}

#[contractimpl]
impl CRUD for Company {}
