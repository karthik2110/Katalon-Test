$(document).ready(function() {var formatter = new CucumberHTML.DOMFormatter($('.cucumber-report'));formatter.uri("C:/Users/karthik paperflite/Katalon Studio/Sample/Include/features/LoginPage.feature");
formatter.feature({
  "name": "LoginPageFeature",
  "description": "",
  "keyword": "Feature"
});
formatter.scenario({
  "name": "To validate the login page with valid user credentials",
  "description": "",
  "keyword": "Scenario"
});
formatter.step({
  "name": "User navigates to Login page",
  "keyword": "Given "
});
formatter.match({
  "location": "LoginPage.NavigatestoLoginPage()"
});
formatter.result({
  "status": "passed"
});
formatter.step({
  "name": "User enters valid credentials",
  "keyword": "When "
});
formatter.match({
  "location": "LoginPage.EnteringValidCredentials()"
});
formatter.result({
  "status": "passed"
});
formatter.step({
  "name": "Click on Login button",
  "keyword": "And "
});
formatter.match({
  "location": "LoginPage.LoginButtonClick()"
});
formatter.result({
  "status": "passed"
});
formatter.step({
  "name": "Verify the home page",
  "keyword": "Then "
});
formatter.match({
  "location": "LoginPage.VerifyHomePage()"
});
formatter.result({
  "status": "passed"
});
formatter.uri("C:/Users/karthik paperflite/Katalon Studio/Sample/Include/features/RealLogin.feature");
formatter.feature({
  "name": "PracticeAutomationLoginPageFeature",
  "description": "",
  "keyword": "Feature"
});
formatter.scenario({
  "name": "To validate the practice automation login page with valid user credentials",
  "description": "",
  "keyword": "Scenario"
});
formatter.step({
  "name": "User navigates to practice automation Login page",
  "keyword": "Given "
});
formatter.match({
  "location": "LoginPage.NavigatestoStudentLoginPage()"
});
formatter.result({
  "status": "passed"
});
formatter.step({
  "name": "User enters valid student credentials",
  "keyword": "When "
});
formatter.match({
  "location": "LoginPage.EnteringValidStudentCredentials()"
});
formatter.result({
  "status": "passed"
});
formatter.step({
  "name": "Click on submit button",
  "keyword": "And "
});
formatter.match({
  "location": "LoginPage.SubmitButtonClick()"
});
formatter.result({
  "status": "passed"
});
formatter.step({
  "name": "Verify the student home page",
  "keyword": "Then "
});
formatter.match({
  "location": "LoginPage.VerifyStudentHomePage()"
});
formatter.result({
  "status": "passed"
});
});