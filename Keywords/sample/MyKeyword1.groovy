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



public class MyKeyword1 {
	@Keyword
	def Applogin () {

		WebUI.navigateToUrl('https://myhubstaging.smdservers.net/')

		WebUI.maximizeWindow()

		WebUI.setText(findTestObject('Object Repository/Muhub_Blue/DefaultLogin/Page_Login - SiteLink myHub/input_Corp Code_Client.CorpCode'),
				'SLQA')

		WebUI.setText(findTestObject('Object Repository/Muhub_Blue/DefaultLogin/Page_Login - SiteLink myHub/input_Location Code_Client.LocationCode'),
				'HS')

		WebUI.setText(findTestObject('Object Repository/Muhub_Blue/DefaultLogin/Page_Login - SiteLink myHub/input_User Name_Client.UserName'),
				'HS')

		WebUI.click(findTestObject('Object Repository/Muhub_Blue/DefaultLogin/Page_Login - SiteLink myHub/input_Password_btn btn-lg btn-block btn-primary'))

		WebUI.setEncryptedText(findTestObject('Object Repository/Muhub_Blue/DefaultLogin/Page_Login - SiteLink myHub/input_Password_Client.Password'),
				'm5Ad+DnEs+o=')

		WebUI.click(findTestObject('Object Repository/Muhub_Blue/DefaultLogin/Page_Login - SiteLink myHub/input_Password_btn btn-lg btn-block btn-primary'))

		WebUI.verifyTextPresent('myHub', false)
	}

	@Keyword
	def Congrats() {
		println ('Test Case is Pass')
	}
}


