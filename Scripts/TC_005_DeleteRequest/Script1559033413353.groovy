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
import com.kms.katalon.core.util.KeywordUtil

Res = WS.sendRequest(findTestObject('DeleteCustomerID', [('customerID') : CID]))

WS.verifyResponseStatusCode(Res, 204)

def header = Res.getHeaderFields()
header.each{
	println(it)
}

def isvalueContained
isvalueContained = header.containsKey(CID)

if(isvalueContained){
	println("The customer key " + CID + " is contained in header")
	KeywordUtil.markPassed("Test Passed cause customer key is contained")
}else{
	println("The customer key  is not contained in header")
	KeywordUtil.markFailed("Test Failed cause customer key is NOT contained")

}





