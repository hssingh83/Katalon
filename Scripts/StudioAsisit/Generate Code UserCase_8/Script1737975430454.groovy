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
 * I have a list of test objects as below:
 * username: Object Repository/OrangeHRM/Login/Page_OrangeHRM/input_Username_username
 * password: Object Repository/OrangeHRM/Login/Page_OrangeHRM/input_Password_password
 * Login button: Object Repository/OrangeHRM/Login/Page_OrangeHRM/button_Login
 * LoginText div element: Object Repository/OrangeHRM/Login/Page_OrangeHRM/h6_Dashboard
 * 
 *   I also have a URL: GlobalVariable.OrangeURL, and two local variables 'username' and 'password'
 *   
 *   1. Open browser to the URL stored in URL
 *   Fill the username and password fileds based on the variables 
 *   Click logon button
 *   Verify that the login page dix exisits with 10s 
 *   close browser
 *   
 */
// Open browser to the specified URL
WebUI.openBrowser(GlobalVariable.OrangeURL)

// Set the username field with the value from the variable
WebUI.setText(findTestObject('Object Repository/OrangeHRM/Login/Page_OrangeHRM/input_Username_username'), username)

// Set the password field with the value from the variable
WebUI.setText(findTestObject('Object Repository/OrangeHRM/Login/Page_OrangeHRM/input_Password_password'), password)

// Click the login button
WebUI.click(findTestObject('Object Repository/OrangeHRM/Login/Page_OrangeHRM/button_Login'))

// Verify that the login text div element exists within 10 seconds
WebUI.verifyElementPresent(findTestObject('Object Repository/OrangeHRM/Login/Page_OrangeHRM/h6_Dashboard'), 10)

// Close the browser
WebUI.closeBrowser()
