import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

Response = WS.sendRequest(findTestObject('UpdateCustomerDynamic', [('customerID') : tc_CID, ('firstName') : tc_fName, ('lastName') : tc_lName
            , ('phone') : tc_phone, ('email') : tc_email, ('address') : tc_address, ('city') : tc_city, ('state') : tc_state
            , ('zipcode') : tc_zip, ('country') : tc_country]))

WS.verifyResponseStatusCode(Response, 204)

Response2 = WS.sendRequest(findTestObject('GetCustomerDynamicID', [('customerID') : tc_CID, ('endpoint') : GlobalVariable.endpoint]))

println(Response2.getResponseText())

WS.verifyElementPropertyValue(Response2, 'country', tc_country)

