def before_all(context):
    # Global setup code that runs before all features
    context.stellar_url = "http://localhost:8000"
    context.network_passphrase = "Test SDF Network ; September 2015"

def after_all(context):
    # Global cleanup code that runs after all features
    pass

def before_feature(context, feature):
    # Setup code that runs before each feature
    pass

def after_feature(context, feature):
    # Cleanup code that runs after each feature
    pass

def before_scenario(context, scenario):
    # Reset any scenario-specific data
    context.employees = []
    context.admin_keypair = None
    context.owner_keypair = None
    context.company_contract_hash = None
    context.usdc_contract_id = None
    context.paygo_contract_id = None
    context.company_account_id = None

def after_scenario(context, scenario):
    # Cleanup any scenario-specific data
    pass 