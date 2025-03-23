Feature: Example Feature
    As a user
    I want to demonstrate Behave functionality
    So that others can understand how to write BDD tests

    Scenario: Basic example scenario
        Given I have set up Behave correctly
        When I run the tests
        Then I should see the test results 

    @smoke @api
    Scenario: Tagged scenario 
        Given I have some data
        Then I can access that data

    Scenario: Data table example
      Given I have the following users:
        | name  | email           |
        | John  | john@email.com  |
        | Alice | alice@email.com | 

    Scenario Outline: Test with multiple values
      Given I have <input>
      When I process it
      Then I should get <output>

      Examples:
        | input | output |
        | 1     | 2      |
        | 2     | 4      | 