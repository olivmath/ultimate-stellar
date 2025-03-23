from behave import given, when, then
from typing import Dict, List

@given('I have set up Behave correctly')
def step_behave_setup(context):
    # Add any setup code here
    pass

@when('I run the tests')
def step_run_tests(context):
    # Add test execution code here
    pass

@then('I should see the test results')
def step_verify_results(context):
    # Add verification code here
    assert True, "Test passed successfully"

@given('I have some data')
def step_have_data(context):
    context.my_data = "test data"

@then('I can access that data')
def step_access_data(context):
    assert context.my_data == "test data"

@given('I have the following users')
def step_have_users(context):
    context.users = []
    for row in context.table:
        context.users.append({
            'name': row['name'],
            'email': row['email']
        })

@given('I have {input}')
def step_have_input(context, input):
    context.input = int(input)

@when('I process it')
def step_process_input(context):
    context.output = context.input * 2

@then('I should get {output}')
def step_verify_output(context, output):
    expected = int(output)
    assert context.output == expected, f"Expected {expected}, but got {context.output}" 