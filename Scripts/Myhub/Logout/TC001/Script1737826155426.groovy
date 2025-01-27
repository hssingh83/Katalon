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

WebUI.navigateToUrl(GlobalVariable.MyhubURL)

WebUI.maximizeWindow()

WebUI.setText(findTestObject('Object Repository/Muhub_Blue/Logout/Page_Login - SiteLink myHub/input_Corp Code_Client.CorpCode'), 
    'SLQA')

WebUI.setText(findTestObject('Object Repository/Muhub_Blue/Logout/Page_Login - SiteLink myHub/input_Location Code_Client.LocationCode'), 
    'HS')

WebUI.setText(findTestObject('Object Repository/Muhub_Blue/Logout/Page_Login - SiteLink myHub/input_User Name_Client.UserName'), 
    'HS')

WebUI.click(findTestObject('Object Repository/Muhub_Blue/Logout/Page_Login - SiteLink myHub/input_Password_btn btn-lg btn-block btn-primary'))

WebUI.setEncryptedText(findTestObject('Object Repository/Muhub_Blue/Logout/Page_Login - SiteLink myHub/input_Password_Client.Password'), 
    'm5Ad+DnEs+o=')

WebUI.click(findTestObject('Object Repository/Muhub_Blue/Logout/Page_Login - SiteLink myHub/input_Password_btn btn-lg btn-block btn-primary'))

WebUI.click(findTestObject('Object Repository/Muhub_Blue/Logout/Page_HS LogoutResult/span_HS_menu-down-arrow'))

WebUI.verifyElementText(findTestObject('Object Repository/Muhub_Blue/Logout/Page_HS LogoutResult/a_Log Out'), 'Log Out')

WebUI.click(findTestObject('Object Repository/Muhub_Blue/Logout/Page_HS LogoutResult/svg_HS_svg-inline--fa fa-caret-down fa-w-10'))

WebUI.verifyElementClickable(findTestObject('Object Repository/Muhub_Blue/Logout/Page_HS LogoutResult/a_Log Out'))

WebUI.verifyElementText(findTestObject('Object Repository/Muhub_Blue/Logout/Page_HS LogoutResult/span_SLQA - HS'), 'SLQA - HS')

WebUI.closeBrowser()

