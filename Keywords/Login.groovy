import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testcase.TestCase
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows

import internal.GlobalVariable

public class Login {

	@Keyword
	def void NavigatestoStudentLoginPage() {

		WebUI.openBrowser('')

		WebUI.maximizeWindow()

		WebUI.navigateToUrl('https://practicetestautomation.com/practice-test-login/')

		WebUI.takeScreenshot()
	}

	@Keyword
	def void EnteringValidStudentCredentials(String Username, String Password) {

		WebUI.setText(findTestObject('Object Repository/Login/Username'), Username)

		WebUI.setText(findTestObject('Object Repository/Login/Password'), Password)

		WebUI.takeScreenshot()
	}

	@Keyword
	def void SubmitButtonClick() {

		WebUI.click(findTestObject('Object Repository/Login/Submit Button'))
	}


	@Keyword
	def void VerifyStudentHomePage() {

		WebUI.verifyElementPresent(findTestObject('Object Repository/Login/Practice Menu Option'),30)

		WebUI.takeScreenshot()
	}
}
