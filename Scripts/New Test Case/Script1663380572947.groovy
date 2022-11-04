import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://www.amazon.in/')

WebUI.click(findTestObject('Object Repository/New Folder/Accounts Menu'))

WebUI.acceptAlert()

WebUI.doubleClick(findTestObject('Object Repository/New Folder/Accounts Menu'))

WebUI.sendKeys(findTestObject('Object Repository/New Folder/Accounts Menu'), Keys.chord(Keys.ARROW_DOWN))

WebUI.acceptAlert()

WebUI.clearText(findTestObject(null))

WebUI.dragAndDropToObject(findTestObject(null), findTestObject(null))

WebUI.dragAndDropByOffset(findTestObject(null), 0, 0)

WebUI.uploadFile(findTestObject('Object Repository/New Folder/Accounts Menu'), 'C:\\Users\\karthik paperflite\\Desktop\\Crackers.txt')

WebUI.verifyElementPresent(findTestObject(null),30)

