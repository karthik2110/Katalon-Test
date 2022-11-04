Feature: PracticeAutomationLoginPageFeature

  Scenario Outline: To validate the practice automation login page with valid user credentials
        
    Given User navigates to practice automation Login page
    When User enters <Username> and <Password>
    And Click on submit button
    Then Verify the student home page

Examples:
 
      | Username| Password |
      | student | Password123| 
      | student | password | 