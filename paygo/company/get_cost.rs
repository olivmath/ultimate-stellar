// Get company total cost (sum of all employee budgets)
pub fn get_company_cost(env: Env, company_name: String) -> Result<i128, Error> {
    let companies = env.storage().instance();
    let key = symbol_short!("company");

    if !companies.has(&key) {
        return Err(Error::CompanyNotFound);
    }

    let company_map: Map<String, Company> = companies.get(&key).unwrap();

    if !company_map.contains_key(&company_name) {
        return Err(Error::CompanyNotFound);
    }

    let company = company_map.get(&company_name).unwrap();

    let mut total_cost: i128 = 0;
    for employee in company.employees.iter() {
        total_cost += employee.budget;
    }

    Ok(total_cost)
}
