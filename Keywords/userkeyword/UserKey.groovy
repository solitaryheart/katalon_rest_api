package userkeyword

import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

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

import internal.GlobalVariable


public class UserKey {

	@Keyword
	def print_welcome_message(){
		WebUI.openBrowser("https://www.facebook.com")
		println "Hello User...No we are ready to write test case"
	}

	@Keyword
	def start_browser_maximize_printmessage(String url){
		WebUI.openBrowser(url)
		WebUI.maximizeWindow()
		println "Cool testcase"
	}

	@Keyword
	def start_browser_maximize_printTitle(String url){
		WebUI.openBrowser(url)
		WebUI.maximizeWindow()
		return WebUI.getWindowTitle()
	}
}
