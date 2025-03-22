// Fund company balance
pub fn fund_company(env: Env, company_name: String, amount: i128) -> Result<(), Error> {
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

    company.balance += amount;
    company.is_active = true;

    company_map.set(company_name.clone(), company);
    companies.set(&key, company_map);

    // Emit funding event
    env.events().publish(
        (symbol_short!("company"), symbol_short!("fund")),
        (company_name, amount),
    );

    Ok(())
}
