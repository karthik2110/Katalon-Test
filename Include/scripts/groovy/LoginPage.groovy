import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

import cucumber.api.java.en.And
import cucumber.api.java.en.Given
import cucumber.api.java.en.Then
import cucumber.api.java.en.When
import steps.CommonSteps
import gherkin.ast.DataTable


class LoginPage {

	@Then("Verify the home page")
	def void VerifyHomePage() {
		println("Homepage is displaying")
	}

	@Given("User navigates to Login page")
	def void NavigatestoLoginPage() {
		println("User successfully navigates to login page")
	}

	@When("User enters valid credentials")
	def void EnteringValidCredentials() {
		println("User entering valid credentials")
	}

	@And("Click on Login button")
	def void LoginButtonClick() {
		println("User clicked login button")
	}

	@Given("User navigates to practice automation Login page")
	def void NavigatestoStudentLoginPage() {

		CustomKeywords.'Login.NavigatestoStudentLoginPage'()
	}

	@When("User enters (.*) and (.*)")
	def void EnteringValidStudentCredentials(String Username, String Password) {

		WebUI.setText(findTestObject('Object Repository/Login/Username'), Username)

		WebUI.setText(findTestObject('Object Repository/Login/Password'), Password)

		WebUI.takeScreenshot()
	}

	@And("Click on submit button")
	def void SubmitButtonClick() {

		CustomKeywords.'Login.SubmitButtonClick'()
	}

	@Then("Verify the student home page")
	def void VerifyStudentHomePage() {

		CustomKeywords.'Login.VerifyStudentHomePage'()
	}

	@Given('I read data row "(.*)" from "(.*)"')
	def IReadRowDataFrom(String dataRow , String table) {
		CommonSteps.testData = findTestData(table)
		CommonSteps.testIndex = dataRow as Integer
		WebUI.comment(":) -> DataRow " + dataRow + " read from " + table + "data table")
	}
}