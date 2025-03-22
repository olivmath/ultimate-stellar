// Get all companies owned by an address
pub fn get_owner_companies(env: Env, owner: Address) -> Vec<String> {
    let companies = env.storage().instance();
    let key = symbol_short!("company");

    if !companies.has(&key) {
        return vec![&env];
    }

    let company_map: Map<String, Company> = companies.get(&key).unwrap();
    let mut owner_companies = vec![&env];

    for (name, company) in company_map.iter() {
        if company.owner == owner {
            owner_companies.push_back(name);
        }
    }

    owner_companies
}
