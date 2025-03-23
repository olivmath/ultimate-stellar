import subprocess
import time
import requests

def start_stellar_node():
    """Start the Stellar node container"""
    cmd = [
        "podman", "run",
        "--rm",
        "-p", "8000:8000",
        "--name", "stellar",
        "stellar/quickstart",
        "--local",
        "--enable-soroban-rpc"
    ]
    
    try:
        # Start the container
        process = subprocess.Popen(cmd)
        
        # Wait for the node to be ready
        print("Waiting for Stellar node to start...")
        for _ in range(60):  # Try for 60 seconds
            try:
                # Try to connect to the HTTP endpoint
                response = requests.get("http://localhost:8000/")
                if response.status_code == 200:
                    print("Stellar node is responding to HTTP...")
                    # Additional wait to ensure all services are up
                    time.sleep(10)
                    return True
            except requests.exceptions.RequestException:
                pass
            time.sleep(1)
            print(".", end="", flush=True)
        
        raise Exception("Timeout waiting for Stellar node to start")
    except Exception as e:
        print(f"Failed to start Stellar node: {str(e)}")
        return False

def stop_stellar_node():
    """Stop the Stellar node container"""
    try:
        subprocess.run(["podman", "stop", "stellar"], check=True)
        print("Stellar node stopped successfully")
    except subprocess.CalledProcessError as e:
        print(f"Failed to stop Stellar node: {str(e)}")

def before_all(context):
    # Global setup code that runs before all features
    context.stellar_url = "http://localhost:8000"
    context.network_passphrase = "Test SDF Network ; September 2015"
    
    # Start Stellar node
    if not start_stellar_node():
        raise Exception("Failed to start Stellar node")

def after_all(context):
    # Global cleanup code that runs after all features
    stop_stellar_node()

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