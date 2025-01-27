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

WebUI.navigateToUrl('https://myhubblue.smdservers.net/Account/Login?ReturnUrl=%2f')

WebUI.click(findTestObject('Object Repository/Muhub_Blue/Login/Page_Login - Invalid/body_(function()function c()var ba.contentD_92d7c6'))

WebUI.setText(findTestObject('Object Repository/Muhub_Blue/Login/Page_Login - Invalid/input_Corp Code_Client.CorpCode'), 'SLQA')

WebUI.setText(findTestObject('Object Repository/Muhub_Blue/Login/Page_Login - Invalid/input_Location Code_Client.LocationCode'), 
    'HS')

WebUI.setText(findTestObject('Object Repository/Muhub_Blue/Login/Page_Login - Invalid/input_User Name_Client.UserName'), 'HS')

WebUI.click(findTestObject('Object Repository/Muhub_Blue/Login/Page_Login - Invalid/input_Password_btn btn-lg btn-block btn-primary'))

WebUI.setEncryptedText(findTestObject('Object Repository/Muhub_Blue/Login/Page_Login - Invalid/input_Password_Client.Password'), 
    '+bLqWn6uy8C4sPka7Y+esQ==')

WebUI.click(findTestObject('Object Repository/Muhub_Blue/Login/Page_Login - Invalid/input_Password_btn btn-lg btn-block btn-primary'))

WebUI.verifyElementText(findTestObject('Object Repository/Muhub_Blue/Login/Page_Login - Invalid/span_ERROR - Invalid Credentials'), 
    'ERROR - Invalid Credentials')

WebUI.closeBrowser()

