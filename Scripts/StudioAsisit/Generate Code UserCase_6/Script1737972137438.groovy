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
 * corpcode: Object Repository/Muhub_Blue/Login/Page_LoginPage/input_Corp Code_Client.CorpCode
 * locationcode: Object Repository/Muhub_Blue/Login/Page_LoginPage/input_Location Code_Client.LocationCode
 * username: Object Repository/Muhub_Blue/Login/Page_LoginPage/input_User Name_Client.UserName
 * password: Object Repository/Muhub_Blue/Login/Page_LoginPage/input_Password_Client.Password
 * Login button: Object Repository/Muhub_Blue/Login/Page_LoginPage/input_Password_btn btn-lg btn-block btn-primary
 * login Text: Object Repository/Muhub_Blue/Login/Page_LoginResult/span_SLQA - HS
 *
 *   I also have a URL: GlobalVariable.MyhubURL, and four local variables 'corpcode','locationcode','username' and 'password'
 *
 *   1. Open browser to the URL stored in MyhubUR
 *   2. Fill the corpcode,locationcode, username based on the variables
 *   3. Click login button
 *   4. Fill the password based on the variables 
 *   5. Click login button
 *   6. Verify that the login page dix exisits with 10s
 *   7. close browser
 *
 */
// Open the browser to the specified URL
WebUI.openBrowser(GlobalVariable.MyhubURL)

// Set the corpcode input field with the value from the variable
WebUI.setText(findTestObject('Muhub_Blue/Login/Page_LoginPage/input_Corp Code_Client.CorpCode'), corpcode)

// Set the locationcode input field with the value from the variable
WebUI.setText(findTestObject('Object Repository/Muhub_Blue/Login/Page_LoginPage/input_Location Code_Client.LocationCode'), locationcode)

// Set the username input field with the value from the variable
WebUI.setText(findTestObject('Object Repository/Muhub_Blue/Login/Page_LoginPage/input_User Name_Client.UserName'), username)

// Click the login button
WebUI.click(findTestObject('Object Repository/Muhub_Blue/Login/Page_LoginPage/input_Password_btn btn-lg btn-block btn-primary'))

// Set the password input field with the value from the variable
WebUI.setText(findTestObject('Object Repository/Muhub_Blue/Login/Page_LoginPage/input_Password_Client.Password'), password)

// Click the login button again
WebUI.click(findTestObject('Object Repository/Muhub_Blue/Login/Page_LoginPage/input_Password_btn btn-lg btn-block btn-primary'))

// Verify that the login page exists within 10 seconds
WebUI.verifyElementPresent(findTestObject('Object Repository/Muhub_Blue/LoginResult/span_SLQA - HS'), 10)

// Close the browser
WebUI.closeBrowser()

