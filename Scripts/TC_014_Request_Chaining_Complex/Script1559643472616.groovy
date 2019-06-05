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
import groovy.json.JsonSlurper as JsonSlurper

Res_Post_Customer = WS.sendRequest(findTestObject('POSTDataDrivenCustomer', [('firstName') : tc_fName, ('lastName') : tc_lName
            , ('phone') : tc_phone, ('email') : tc_email, ('address') : tc_address, ('city') : tc_city, ('state') : tc_state
            , ('zipcode') : tc_zip, ('country') : tc_country]))

//println(Res_Post_Customer.getResponseText())
def jsonS = new JsonSlurper()

def jsonResponse = jsonS.parseText(Res_Post_Customer.getResponseText())

println(jsonResponse.customerID)

fetched_CID = jsonResponse.customerID

println(fetched_CID)

Res_Get_Customer = WS.sendRequest(findTestObject('GetCustomerDynamicID', [('customerID') : fetched_CID, ('endpoint') : GlobalVariable.endpoint]))

WS.verifyElementPropertyValue(Res_Get_Customer, 'customerID', fetched_CID)

Res_Create_Order = WS.sendRequest(findTestObject('CreateOrder', [('customerID') : fetched_CID]))

def jsonSOrder = new JsonSlurper()

def jsonResponseOrder = jsonSOrder.parseText(Res_Create_Order.getResponseText())

fetchedORDERID = jsonResponseOrder.orderID

println(fetchedORDERID)

WS.verifyElementPropertyValue(Res_Create_Order, 'orderStatus', 'Order Created')

Res_Get_Order = WS.sendRequest(findTestObject('GetOrder', [('customerID') : fetched_CID, ('orderID') : fetched_ORDERID]))

println(Res_Get_Order.getResponseText())

def jsonS3 = new JsonSlurper()

def jsonResponse3 = jsonS3.parseText(Res_Get_Order.getResponseText())

println(jsonResponse3.orderID)

WS.verifyElementPropertyValue(findTestObject(null), '', null)

