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

WebUI.setText(findTestObject('Object Repository/Muhub_Blue/Operation/moveIn/Page_Login - SiteLink myHub/input_Corp Code_Client.CorpCode'), 
    'SLQA')

WebUI.setText(findTestObject('Object Repository/Muhub_Blue/Operation/moveIn/Page_Login - SiteLink myHub/input_Location Code_Client.LocationCode'), 
    'HS')

WebUI.setText(findTestObject('Object Repository/Muhub_Blue/Operation/moveIn/Page_Login - SiteLink myHub/input_User Name_Client.UserName'), 
    'HS')

WebUI.click(findTestObject('Object Repository/Muhub_Blue/Operation/moveIn/Page_Login - SiteLink myHub/input_Password_btn btn-lg btn-block btn-primary'))

WebUI.setEncryptedText(findTestObject('Object Repository/Muhub_Blue/Operation/moveIn/Page_Login - SiteLink myHub/input_Password_Client.Password'), 
    'm5Ad+DnEs+o=')

WebUI.click(findTestObject('Object Repository/Muhub_Blue/Operation/moveIn/Page_Login - SiteLink myHub/input_Password_btn btn-lg btn-block btn-primary'))

WebUI.verifyElementText(findTestObject('Object Repository/Muhub_Blue/Operation/moveIn/Page_HS  Home - SiteLink myHub/a_Operations'), 
    'Operations')

WebUI.verifyElementVisible(findTestObject('Object Repository/Muhub_Blue/Operation/moveIn/Page_HS  Home - SiteLink myHub/a_Operations'))

WebUI.verifyElementClickable(findTestObject('Object Repository/Muhub_Blue/Operation/moveIn/Page_HS  Home - SiteLink myHub/a_Operations'))

WebUI.verifyElementText(findTestObject('Object Repository/Muhub_Blue/Operation/moveIn/Page_HS  Home - SiteLink myHub/a_Move In'), 
    'Move In')

WebUI.verifyElementVisible(findTestObject('Object Repository/Muhub_Blue/Operation/moveIn/Page_HS  Home - SiteLink myHub/a_Move In'))

WebUI.verifyElementClickable(findTestObject('Object Repository/Muhub_Blue/Operation/moveIn/Page_HS  Home - SiteLink myHub/a_Move In'))

WebUI.click(findTestObject('Muhub_Blue/Operation/moveIn/Page_HS  Home - SiteLink myHub/a_Move In'))

WebUI.verifyElementText(findTestObject('Object Repository/Muhub_Blue/Operation/moveIn/Page_HS  Move In - Select Unit - SiteLink myHub/div_Move In'), 
    'Move In')

WebUI.closeBrowser()

