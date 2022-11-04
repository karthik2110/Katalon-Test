import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKWF
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.webui.keyword.internal.WebUIAbstractKeyword
import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.CheckpointFactory
import com.kms.katalon.core.testcase.TestCaseFactory
import com.kms.katalon.core.testdata.TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository

import cucumber.api.Scenario
import cucumber.api.java.en.And
import cucumber.api.java.en.Given
import cucumber.api.java.en.Then
import cucumber.api.java.en.When
import org.openqa.selenium.JavascriptExecutor
import org.openqa.selenium.Keys
import org.openqa.selenium.WebDriver
import org.openqa.selenium.WebElement

import cucumber.api.java.Before
import cucumber.api.java.After

import com.kms.katalon.core.webui.common.WebUiCommonHelper
import com.kms.katalon.core.testdata.ExcelData as ExcelData
import internal.GlobalVariable as GlobalVariable

import steps.CommonSteps


public class WebSteps {
	
	@Given('I read data row "(.*)" from "(.*)"')
	def IReadRowDataFrom(String dataRow , String table) {
		CommonSteps.testData = findTestData(table)
		CommonSteps.testIndex = dataRow as Integer
		WebUI.comment(":) -> DataRow " + dataRow + " read from " + table + "data table")
	}

}