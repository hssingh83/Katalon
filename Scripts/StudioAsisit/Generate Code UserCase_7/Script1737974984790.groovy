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

/*
 * Write a Katalon test studio to perform following action
 * Open bowser to the url stored in the GlobalVariable.OrangeURL
 * Fill in the username 'Object Repository/OrangeHRM/Login/Page_OrangeHRM/input_Username_username' with in the value of local variable 'username'
 * Fill in the password 'Object Repository/OrangeHRM/Login/Page_OrangeHRM/input_Password_password' with in the value of local variable 'password'
 * Click login button with the id 'Object Repository/OrangeHRM/Login/Page_OrangeHRM/button_Login'
 * Verify that the login text div 'Object Repository/OrangeHRM/Login/Page_OrangeHRM/h6_Dashboard' exisits with in timeout 10s
 * close the browser*
 */
// Open the browser to the specified URL
WebUI.openBrowser(GlobalVariable.OrangeURL)

// Fill in the username field with the value of the local variable 'username'
WebUI.setText(findTestObject('Object Repository/OrangeHRM/Login/Page_OrangeHRM/input_Username_username'), username)

// Fill in the password field with the value of the local variable 'password'
WebUI.setText(findTestObject('Object Repository/OrangeHRM/Login/Page_OrangeHRM/input_Password_password'), password)

// Click the login button
WebUI.click(findTestObject('Object Repository/OrangeHRM/Login/Page_OrangeHRM/button_Login'))

// Verify that the login text div exists within a timeout of 10 seconds
WebUI.verifyElementPresent(findTestObject('Object Repository/OrangeHRM/Login/Page_OrangeHRM/h6_Dashboard'), 10)

// Close the browser
WebUI.closeBrowser()