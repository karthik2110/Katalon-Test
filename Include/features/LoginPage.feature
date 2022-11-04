Feature: LoginPageFeature

  Scenario: To validate the login page with valid user credentials
    Given User navigates to Login page
    When User enters valid credentials
    And Click on Login button
    Then Verify the home page
