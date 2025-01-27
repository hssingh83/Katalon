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

WebUI.callTestCase(findTestCase('Myhub/Login/TC000'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.verifyElementText(findTestObject('Object Repository/Muhub_Blue/Billing/Invoice/Page_HS  Home - SiteLink myHub/a_Billing'), 
    'Billing')

WebUI.verifyElementVisible(findTestObject('Object Repository/Muhub_Blue/Billing/Invoice/Page_HS  Home - SiteLink myHub/a_Billing'))

WebUI.verifyElementClickable(findTestObject('Object Repository/Muhub_Blue/Billing/Invoice/Page_HS  Home - SiteLink myHub/a_Billing'))

WebUI.verifyElementText(findTestObject('Object Repository/Muhub_Blue/Billing/Invoice/Page_HS  Home - SiteLink myHub/a_Invoices'), 
    'Invoices')

WebUI.verifyElementVisible(findTestObject('Object Repository/Muhub_Blue/Billing/Invoice/Page_HS  Home - SiteLink myHub/a_Invoices'))

WebUI.verifyElementClickable(findTestObject('Object Repository/Muhub_Blue/Billing/Invoice/Page_HS  Home - SiteLink myHub/a_Invoices'))

WebUI.click(findTestObject('Object Repository/Muhub_Blue/Billing/Invoice/Page_HS  Home - SiteLink myHub/a_Invoices'))

WebUI.getUrl()

WebUI.verifyElementText(findTestObject('Object Repository/Muhub_Blue/Billing/Invoice/Page_HS  Billing - Process Invoices - SiteL_0c1854/div_Invoices'), 
    'Invoices')

WebUI.closeBrowser()

