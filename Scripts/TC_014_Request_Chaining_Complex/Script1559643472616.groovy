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

Res_Post_Customer = WS.sendRequest(findTestObject('POSTDataDrivenCustomer', [('firstName') : 'gerd', ('lastName') : 'Podolski'
            , ('phone') : 10006, ('email') : 'lp10@kps.com', ('address') : 'Parkstrasse5', ('city') : 'Koeln', ('state') : 'NRW'
            , ('zipcode') : 45000, ('country') : 'Germany']))

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

print(jsonResponseOrder)

fetchedORDERID = jsonResponseOrder.orderID

GlobalVariable.OrderID = jsonResponseOrder.orderID

println(fetchedORDERID)

print(GlobalVariable.OrderID)

WS.verifyElementPropertyValue(Res_Create_Order, 'orderStatus', 'Order Created')

Res_Get_Order = WS.sendRequest(findTestObject('GetOrder', [('customerID') : fetched_CID, ('orderID') : fetched_ORDERID]))

println(Res_Get_Order.getResponseText())

WS.verifyResponseStatusCode(Res_Get_Order, 200)

println('Before sending request' + fetchedORDERID)

4.times({ 
        Res_Create_Product = WS.sendRequest(findTestObject('CreateProduct', [('myorderID') : GlobalVariable.OrderID, ('customerID') : fetched_CID
                    , ('productName') : 'G30DrivingBelt', ('cost') : 250, ('trackingID') : 'ID-0-16']))

        WS.verifyResponseStatusCode(Res_Create_Product, 201)
    })

Res_Get_Customer_Products = WS.sendRequest(findTestObject('GetCustomerDynamicID', [('customerID') : fetched_CID, ('endpoint') : GlobalVariable.endpoint]))

WS.verifyElementsCount(Res_Get_Customer_Products, 'orders[0].products', 4)

Res_Delete_Customer = WS.sendRequest(findTestObject('DeleteCustomerID', [('customerID') : fetched_CID]))

WS.verifyResponseStatusCode(Res_Delete_Customer, 204)

