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

def slurper = new groovy.json.JsonSlurper()

response_0 = WS.sendRequest(findTestObject('Sub_Postman/1. Login/Login/2.1 Login', [('link') : GlobalVariable.link, ('phone') : phoneLogin1
	, ('password') : password1, ('deviceId') : deviceId1, ('firebaseToken') : firebaseToken1]))
def result_0= slurper.parseText(response_0.getResponseBodyContent())
def errCode_0 = result_0.err
println(errCode_0)
def messageFail_0 = result_0.message
println(messageFail_0)
GlobalVariable.tokenCus = result_0.token.token
println(result_0.token.token)
//
response = WS.sendRequest(findTestObject('Sub_Postman/Home/Request Transfer/Request Transfer',
	[('link') : GlobalVariable.link
	,('tokenCus') : GlobalVariable.tokenCus,
	 ('phone') : phone,
	 ('amount') : amount,
	 ('currency') : currency,
	 ('message') : message,
	]))

def result= slurper.parseText(response.getResponseBodyContent())
def errCode = result.err
println(errCode)
def messageErr = result.message
println(messageErr)
WS.verifyElementPropertyValue(response, 'message',MessageFail)
WS.verifyElementPropertyValue(response, 'err',Expected_Err)
