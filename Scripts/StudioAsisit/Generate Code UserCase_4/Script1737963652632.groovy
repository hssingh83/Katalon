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
 * Open bowser to the url stored in the GlobalVariable.URL
 * Click the make appointment button with the id 'Object Repository/Cura_healthcare/Logout/Page_CURA Healthcare Service/a_Make Appointment'
 * Fill in the username 'Object Repository/Cura_healthcare/Logout/Page_CURA Healthcare Service/input_Username_username' with in the value of local variable 'username'
 * Fill in the password 'Object Repository/Cura_healthcare/Logout/Page_CURA Healthcare Service/input_Password_password' with in the value of local variable 'password'
 * Click login button with the id 'Object Repository/Cura_healthcare/Logout/Page_CURA Healthcare Service/button_Login'
 * Click Icon button with the id 'Object Repository/Cura_healthcare/Logout/Page_CURA Healthcare Service/i_CURA Healthcare_fa fa-bars'
 * Verify that the logout icon 'Object Repository/Cura_healthcare/Logout/Page_CURA Healthcare Service/a_Logout' exisits with in timeout 10s
 * Click login button with the id 'Object Repository/Cura_healthcare/Logout/Page_CURA Healthcare Service/a_Logout'
 * close the browser*
 */
// Open the browser to the URL stored in GlobalVariable.URL
WebUI.openBrowser(GlobalVariable.URL)

// Click the make appointment button
WebUI.click(findTestObject('Cura_healthcare/Logout/Page_CURA Healthcare Service/a_Make Appointment'))

// Fill in the username with the value of local variable 'username'
WebUI.setText(findTestObject('Cura_healthcare/Logout/Page_CURA Healthcare Service/input_Username_username'), username)

// Fill in the password with the value of local variable 'password'
WebUI.setText(findTestObject('Cura_healthcare/Logout/Page_CURA Healthcare Service/input_Password_password'), password)

// Click the login button
WebUI.click(findTestObject('Cura_healthcare/Logout/Page_CURA Healthcare Service/button_Login'))

// Click the icon button
WebUI.click(findTestObject('Cura_healthcare/Logout/Page_CURA Healthcare Service/i_CURA Healthcare_fa fa-bars'))

// Verify that the logout icon exists within a timeout of 10 seconds
WebUI.verifyElementPresent(findTestObject('Cura_healthcare/Logout/Page_CURA Healthcare Service/a_Logout'), 10)

// Click the logout button
WebUI.click(findTestObject('Cura_healthcare/Logout/Page_CURA Healthcare Service/a_Logout'))

// Close the browser
WebUI.closeBrowser()


