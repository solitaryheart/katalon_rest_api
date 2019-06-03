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
import groovy.json.JsonSlurper

response1 = WS.sendRequest(findTestObject('AddCustomerID'))

def slurper = new JsonSlurper()

def result = slurper.parseText(response1.getResponseBodyContent())

def value = result.customerID //JSON PATH

//Setting the customerID as globalVariable
GlobalVariable.customerID = value

println('The Global Value is ' + GlobalVariable.customerID)

WS.verifyResponseStatusCode(response1, 201)

WS.verifyElementPropertyValue(response1, 'country', 'England')

response2 = WS.sendRequest(findTestObject('GetCustomerDynamicID'))

println(response2.getResponseText())

