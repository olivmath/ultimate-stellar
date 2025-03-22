#![cfg(test)]

use super::*;
use crate::errors::Error;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn test_company_registration_and_employee_management() {
    let env = Env::default();
    let contract_id = env.register(PayGoContract, ());
    let client = PayGoContractClient::new(&env, &contract_id);

    // Create test addresses
    let owner = Address::random(&env);
    let employee = Address::random(&env);

    // Test company registration
    env.as_contract(&contract_id, || {
        env.set_invoker(owner.clone());

        client
            .register_company(
                &String::from_str(&env, "TechCorp"),
                &String::from_str(&env, "A tech company"),
                &String::from_str(&env, "John Doe"),
                &employee,
                &1000,
            )
            .unwrap();
    });

    // Test adding another employee
    env.as_contract(&contract_id, || {
        env.set_invoker(owner.clone());

        let new_employee = Address::random(&env);
        client
            .add_employee(
                &String::from_str(&env, "TechCorp"),
                &String::from_str(&env, "Jane Smith"),
                &new_employee,
                &1500,
            )
            .unwrap();
    });

    // Test getting company cost
    env.as_contract(&contract_id, || {
        let total_cost = client
            .get_company_cost(&String::from_str(&env, "TechCorp"))
            .unwrap();
        assert_eq!(total_cost, 2500); // 1000 + 1500
    });

    // Test funding company
    env.as_contract(&contract_id, || {
        env.set_invoker(owner.clone());

        client
            .fund_company(&String::from_str(&env, "TechCorp"), &3000)
            .unwrap();
    });

    // Test paying employees
    env.as_contract(&contract_id, || {
        env.set_invoker(owner.clone());

        client
            .pay_employees(&String::from_str(&env, "TechCorp"))
            .unwrap();
    });

    // Test getting owner companies
    env.as_contract(&contract_id, || {
        let companies = client.get_owner_companies(&owner);
        assert_eq!(companies.len(), 1);
        assert_eq!(
            companies.get(0).unwrap(),
            String::from_str(&env, "TechCorp")
        );
    });
}

#[test]
fn test_error_cases() {
    let env = Env::default();
    let contract_id = env.register(PayGoContract, ());
    let client = PayGoContractClient::new(&env, &contract_id);

    let owner = Address::random(&env);
    let unauthorized_user = Address::random(&env);
    let employee = Address::random(&env);

    // Register a company first
    env.as_contract(&contract_id, || {
        env.set_invoker(owner.clone());

        client
            .register_company(
                &String::from_str(&env, "TechCorp"),
                &String::from_str(&env, "A tech company"),
                &String::from_str(&env, "John Doe"),
                &employee,
                &1000,
            )
            .unwrap();
    });

    // Test unauthorized access
    env.as_contract(&contract_id, || {
        env.set_invoker(unauthorized_user.clone());

        let result = client.add_employee(
            &String::from_str(&env, "TechCorp"),
            &String::from_str(&env, "Jane Smith"),
            &Address::random(&env),
            &1500,
        );
        assert_eq!(result, Err(Error::Unauthorized));
    });

    // Test company not found
    env.as_contract(&contract_id, || {
        env.set_invoker(owner.clone());

        let result = client.get_company_cost(&String::from_str(&env, "NonExistentCorp"));
        assert_eq!(result, Err(Error::CompanyNotFound));
    });

    // Test insufficient funds
    env.as_contract(&contract_id, || {
        env.set_invoker(owner.clone());

        let result = client.pay_employees(&String::from_str(&env, "TechCorp"));
        assert_eq!(result, Err(Error::InsufficientFunds));
    });

    // Test duplicate company registration
    env.as_contract(&contract_id, || {
        env.set_invoker(owner.clone());

        let result = client.register_company(
            &String::from_str(&env, "TechCorp"),
            &String::from_str(&env, "Another tech company"),
            &String::from_str(&env, "John Doe"),
            &employee,
            &1000,
        );
        assert_eq!(result, Err(Error::CompanyAlreadyExists));
    });
}
