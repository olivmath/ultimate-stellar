#![no_std]
use soroban_sdk::contract;

mod errors;
mod types;

#[contract]

pub struct PayGoContract;

mod test;
