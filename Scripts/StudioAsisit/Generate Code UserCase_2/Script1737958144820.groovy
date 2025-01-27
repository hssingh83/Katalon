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
 * Open bowser to the url stored in the GlobalVariable.MyhubURL
 * Fill the Cropcode 'Object Repository/Muhub_Blue/Login/Page_LoginPage/input_Corp Code_Client.CorpCode' with the value of the local variable 'cropcode'
 * Fill the Locationcode 'Object Repository/Muhub_Blue/Login/Page_LoginPage/input_Location Code_Client.LocationCode' with the value of the local variable 'location'
 * Fill the Locationcode 'Object Repository/Muhub_Blue/Login/Page_LoginPage/input_User Name_Client.UserName' with the value of the local variable 'username'
 * Fill the Username 'Object Repository/Muhub_Blue/Login/Page_LoginPage/input_Password_Client.Password' with the value of the local variable 'password'
 * click login button with the value 'Object Repository/Muhub_Blue/Login/Page_LoginPage/input_Password_btn btn-lg btn-block btn-primary'
 * Verify the login 'Object Repository/Muhub_Blue/Login/Page_LoginResult/span_SLQA - HS' exists with timeout 10s
 * close the browser*
 */
 
// Open the browser to the specified URL
WebUI.openBrowser(GlobalVariable.MyhubURL)

// Set the value of the crop code input field
WebUI.setText(findTestObject('Muhub_Blue/Login/Page_LoginPage/input_Corp Code_Client.CorpCode'), cropcode)

// Set the value of the location code input field
WebUI.setText(findTestObject('Muhub_Blue/Login/Page_LoginPage/input_Location Code_Client.LocationCode'), location)

// Set the value of the username input field
WebUI.setText(findTestObject('Muhub_Blue/Login/Page_LoginPage/input_User Name_Client.UserName'), username)

// Click the login button
WebUI.click(findTestObject('Muhub_Blue/Login/Page_LoginPage/input_Password_btn btn-lg btn-block btn-primary'))

// Set the value of the password input field
WebUI.setText(findTestObject('Muhub_Blue/Login/Page_LoginPage/input_Password_Client.Password'), password)

// Click the login button
WebUI.click(findTestObject('Muhub_Blue/Login/Page_LoginPage/input_Password_btn btn-lg btn-block btn-primary'))

// Verify that the login result element exists within a timeout of 10 seconds
WebUI.verifyElementPresent(findTestObject('Muhub_Blue/LoginResult/span_SLQA - HS'), 10)

// Close the browser
WebUI.closeBrowser()
