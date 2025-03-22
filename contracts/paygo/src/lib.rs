#![no_std]

mod test;

use soroban_sdk::{contract, contractimpl, vec, Env, String, Vec};

#[contract]
pub struct PayGo;

#[contractimpl]
impl PayGo {
    pub fn create_company(e: Env) {

        //  # Validate

        // company must:
        // - not empty
        // - not has any employee with invalid account id
        // - not has duplicated employee in list
        // owner must:
        // - have balance enogh to cover total cost
        // - pay with `USDC` token

        //  # Write

        // get wasm hash
        // mount company with data
        // instantiate company in blockchain
        // transfer balance from owner to company contract

        // # Return company id
    }

    pub fn pay_employees(e: Env) {}
}

// CASE 1
// 1 Owner add company
// 1.1 Company must have: name, description, list of employee, account_id, owner
// 1.1.1 Employee must have: name, account_id, budget
// 1.2 Owner fund company
// 1.3 Owner call function and pay a percent of budget to all employees

// CASE 2
// 2. Owner remove company
// 2.1 Send back money to of balance to owner
// 2.2 company deleted

// CASE 3
// 3. Owner edit company
// 3.1 Owner add/edit/remove employee on some company
// 3.2 Company recalcule total cost
// 3.3 Owner call function and pay a percent of budget to all employees
