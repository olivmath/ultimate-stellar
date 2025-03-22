// Register a new company
pub fn register_company(
    env: Env,
    name: String,
    description: String,
    initial_employee_name: String,
    initial_employee_account: Address,
    initial_employee_budget: i128,
) -> Result<(), Error> {
    let owner = env.invoker();

    // Create initial employee
    let employee = Employee {
        name: initial_employee_name,
        account_id: initial_employee_account,
        budget: initial_employee_budget,
        last_payment: env.ledger().timestamp(),
    };

    let company = Company {
        name: name.clone(),
        description,
        employees: vec![&env, employee],
        owner: owner.clone(),
        balance: 0,
        is_active: false,
    };

    // Store company data
    let companies = env.storage().instance();
    let key = symbol_short!("company");

    // Check if company already exists
    if companies.has(&key) {
        let mut company_map: Map<String, Company> = companies.get(&key).unwrap();
        if company_map.contains_key(&name) {
            return Err(Error::CompanyAlreadyExists);
        }
        company_map.set(name, company);
        companies.set(&key, company_map);
    } else {
        let mut company_map = Map::new(&env);
        company_map.set(name, company);
        companies.set(&key, company_map);
    }

    // Emit company registration event
    env.events()
        .publish((symbol_short!("company"), symbol_short!("register")), owner);

    Ok(())
}
