from behave import given, when, then
from stellar_sdk import Server, Keypair, TransactionBuilder, Network, Asset
from typing import List, Dict
import random
import requests

# Helper functions
def generate_random_account_id() -> str:
    """Generate a random Stellar account ID"""
    return Keypair.random().public_key

def create_employee_list(count: int, total_budget: float) -> List[Dict]:
    """Create a list of employees with random account IDs and distributed budget"""
    employees = []
    # Calculate base salary (equal distribution for simplicity)
    base_salary = total_budget / count
    
    for i in range(count):
        employees.append({
            'name': f'Employee_{i+1}',
            'account_id': generate_random_account_id(),
            'budget': base_salary
        })
    return employees

@given('the Stellar network is running')
def step_check_stellar_network(context):
    """Verify Stellar network is accessible"""
    try:
        context.server = Server("http://localhost:8000")
        context.network = Network.TESTNET_NETWORK_PASSPHRASE
        # Test connection
        context.server.load_account(Keypair.random().public_key)
    except Exception as e:
        raise AssertionError(f"Stellar network not accessible: {str(e)}")

@given('I have an admin wallet funded')
def step_setup_admin_wallet(context):
    """Setup and fund admin wallet"""
    context.admin_keypair = Keypair.random()
    # Fund the admin account using friendbot (testnet) or local network
    try:
        requests.get(f"http://localhost:8000/friendbot?addr={context.admin_keypair.public_key}")
    except Exception as e:
        raise AssertionError(f"Could not fund admin wallet: {str(e)}")

@given('I have an owner wallet funded')
def step_setup_owner_wallet(context):
    """Setup and fund owner wallet"""
    context.owner_keypair = Keypair.random()
    # Fund the owner account
    try:
        requests.get(f"http://localhost:8000/friendbot?addr={context.owner_keypair.public_key}")
    except Exception as e:
        raise AssertionError(f"Could not fund owner wallet: {str(e)}")

@given('I have a list of 100 employees with total budget of 100K USDC')
def step_create_employee_list(context):
    """Create list of 100 employees with total budget of 100K USDC"""
    context.employees = create_employee_list(100, 100000)
    assert len(context.employees) == 100, "Failed to create 100 employees"
    total_budget = sum(emp['budget'] for emp in context.employees)
    assert abs(total_budget - 100000) < 0.01, "Total budget does not match 100K USDC"

@given('admin uploads the company contract to Stellar')
def step_upload_company_contract(context):
    """Upload company contract to Stellar"""
    # Implementation depends on your contract deployment method
    # This is a placeholder for the actual contract deployment
    context.company_contract_hash = "company_contract_hash"
    assert context.company_contract_hash, "Failed to upload company contract"

@given('admin uploads and instantiates the USDC contract')
def step_setup_usdc_contract(context):
    """Upload and instantiate USDC contract"""
    # Implementation depends on your contract deployment method
    context.usdc_contract_id = "usdc_contract_id"
    assert context.usdc_contract_id, "Failed to setup USDC contract"

@given('admin uploads and instantiates the PayGo contract')
def step_setup_paygo_contract(context):
    """Upload and instantiate PayGo contract"""
    # Implementation depends on your contract deployment method
    context.paygo_contract_id = "paygo_contract_id"
    assert context.paygo_contract_id, "Failed to setup PayGo contract"

@when('owner approves 100K USDC to PayGo contract')
def step_approve_usdc(context):
    """Approve USDC spending to PayGo contract"""
    # Implementation depends on your contract interaction method
    success = True  # Replace with actual approval call
    assert success, "Failed to approve USDC"

@when('owner creates a company with the employee list')
def step_create_company(context):
    """Create company with employee list"""
    # Implementation depends on your contract interaction method
    context.company_name = "Test Company"
    context.company_description = "Test Description"
    # Call create_company with the employee list
    success = True  # Replace with actual contract call
    assert success, "Failed to create company"

@when('owner retrieves the company contract account ID')
def step_get_company_account_id(context):
    """Get company contract account ID"""
    # Implementation depends on your contract interaction method
    context.company_account_id = "company_account_id"
    assert context.company_account_id, "Failed to get company account ID"

@when('owner calls pay_employees on the company contract')
def step_pay_employees(context):
    """Execute pay_employees function"""
    # Implementation depends on your contract interaction method
    success = True  # Replace with actual contract call
    assert success, "Failed to execute payment"

@then('all 100 employee wallets should receive their correct payment')
def step_verify_payments(context):
    """Verify all employees received correct payment"""
    for employee in context.employees:
        # Get balance of employee account
        try:
            account = context.server.load_account(employee['account_id'])
            balance = float(account.balances[0].balance)  # Assuming USDC is first balance
            expected_balance = employee['budget']
            assert abs(balance - expected_balance) < 0.01, f"Incorrect payment for employee {employee['name']}"
        except Exception as e:
            raise AssertionError(f"Failed to verify payment for {employee['name']}: {str(e)}") 