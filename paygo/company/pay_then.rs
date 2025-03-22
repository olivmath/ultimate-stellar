// Pay all employees of a company
pub fn pay_employees(env: Env, company_name: String) -> Result<(), Error> {
    let admin = env.invoker();

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

    // Verify admin
    if company.owner != admin {
        return Err(Error::Unauthorized);
    }

    // Check if company is active and has sufficient balance
    if !company.is_active {
        return Err(Error::Unauthorized);
    }

    let mut total_payment: i128 = 0;
    for employee in company.employees.iter() {
        total_payment += employee.budget;
    }

    if company.balance < total_payment {
        return Err(Error::InsufficientFunds);
    }

    // Process payments
    for employee in company.employees.iter() {
        // Here you would typically integrate with a token contract to transfer the funds
        // For now we'll just reduce the company balance
        company.balance -= employee.budget;
    }

    company_map.set(company_name.clone(), company);
    companies.set(&key, company_map);

    // Emit payment event
    env.events().publish(
        (symbol_short!("company"), symbol_short!("pay")),
        company_name,
    );

    Ok(())
}
