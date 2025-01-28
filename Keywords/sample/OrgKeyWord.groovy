package sample

import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testcase.TestCase
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows

import internal.GlobalVariable

public class OrgKeyWord {
	@Keyword
	def Applogin () {

		WebUI.navigateToUrl('https://opensource-demo.orangehrmlive.com/')
		WebUI.maximizeWindow()
		WebUI.setText(findTestObject('Object Repository/OrangeHRM/Login/Page_OrangeHRM/input_Username_username'), 'Admin')
		WebUI.setText(findTestObject('Object Repository/OrangeHRM/Login/Page_OrangeHRM/input_Password_password'), 'admin123')
		WebUI.click(findTestObject('Object Repository/OrangeHRM/Login/Page_OrangeHRM/button_Login'))
		WebUI.click(findTestObject('Object Repository/OrangeHRM/Login/Page_OrangeHRM/i_Upgrade_oxd-icon bi-caret-down-fill oxd-u_ca92f9'))
		WebUI.verifyElementPresent(findTestObject('Object Repository/OrangeHRM/Login/Page_OrangeHRM/a_Logout'), 30)
		WebUI.verifyElementClickable(findTestObject('Object Repository/OrangeHRM/Login/Page_OrangeHRM/a_Logout'))
	}

	@Keyword
	def congrats () {

		println ("Test Case is Pass")
	}
}
