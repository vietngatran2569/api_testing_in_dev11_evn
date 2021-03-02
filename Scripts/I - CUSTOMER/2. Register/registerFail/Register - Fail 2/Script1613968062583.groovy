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
response = WS.sendRequest(findTestObject('Sub_Postman/1. Login/Register/1.1 REGISTERUSER', [('link') : GlobalVariable.link
	, ('phone') : phone, ('deviceId') : deviceId]))

response_2 = WS.sendRequest(findTestObject('Sub_Postman/1. Login/Register/1.2 registerVerifyOTP', [('link') : GlobalVariable.link
	, ('phone') : phone, ('otp') : otp, ('deviceId') : deviceId]))

def slurper = new groovy.json.JsonSlurper()
def result2 = slurper.parseText(response_2.getResponseBodyContent())

def errCode = result2.err
println(errCode)
def messageFail = result2.message
println(messageFail)

WS.verifyElementPropertyValue(response_2, 'err', Expected_Err)
WS.verifyElementPropertyValue(response_2, 'message', MessageFail)