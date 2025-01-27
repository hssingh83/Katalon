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
 * Make appointment button : Object Repository/Cura_healthcare/Login/a_Make Appointment
 * Username:Object Repository/Cura_healthcare/Login/input_Username_username
 * Password:Object Repository/Cura_healthcare/Login/input_Password_password
 * Login button: Object Repository/Cura_healthcare/Login/button_Login
 * Appointment div element: Object Repository/Cura_healthcare/Login/h2_Make Appointment
 * 
 *   I also have a URL: GlobalVariable.URL, and two local variables 'Username' and 'Password'
 *   
 *   1. Open browser to the URL stored in URL
 *   click and make appointment button
 *   Fill the username and password fileds based on the variables 
 *   Click logon button
 *   Verify that the login page dix exisits with 10s 
 *   close browser
 *   
 */
// Open the browser to the specified URL
WebUI.openBrowser(GlobalVariable.URL)

// Click the "Make Appointment" button
WebUI.click(findTestObject('Object Repository/Cura_healthcare/Login/a_Make Appointment'))

// Set the username in the username field
WebUI.setText(findTestObject('Object Repository/Cura_healthcare/Login/input_Username_username'), Username)

// Set the password in the password field
WebUI.setText(findTestObject('Object Repository/Cura_healthcare/Login/input_Password_password'), Password)

// Click the login button
WebUI.click(findTestObject('Object Repository/Cura_healthcare/Login/button_Login'))

// Verify that the appointment div element exists within 10 seconds
WebUI.verifyElementPresent(findTestObject('Object Repository/Cura_healthcare/Login/h2_Make Appointment'), 10)

// Close the browser
WebUI.closeBrowser()
