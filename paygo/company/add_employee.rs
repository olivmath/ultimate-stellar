// Add an employee to a company
pub fn add_employee(
    env: Env,
    company_name: String,
    employee_name: String,
    employee_account: Address,
    employee_budget: i128,
) -> Result<(), Error> {
    let owner = env.invoker();

    let companies = env.storage().instance();
    let key = symbol_short!("company");

    if !companies.has(&key) {
        return Err(Error::CompanyNotFound);
    }

    let mut company_map: Map<String, Company> = companies.get(&key).unwrap();

    if !company_map.contains_key(&company_name) {
        return Err(Error::CompanyNotFound);
    }

    let mut company = company_map.get(&company_name).unwrap();

    // Verify owner
    if company.owner != owner {
        return Err(Error::Unauthorized);
    }

    let new_employee = Employee {
        name: employee_name.clone(),
        account_id: employee_account,
        budget: employee_budget,
        last_payment: env.ledger().timestamp(),
    };

    company.employees.push_back(new_employee);
    company_map.set(company_name, company);
    companies.set(&key, company_map);

    // Emit employee added event
    env.events().publish(
        (symbol_short!("employee"), symbol_short!("add")),
        employee_name,
    );

    Ok(())
}
