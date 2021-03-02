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

response_0 =  WS.sendRequest(findTestObject('Object Repository/Agent_Postman/New Folder/Login/Login With UserId', [('link') : GlobalVariable.link, ('phone') : phone
	, ('password') : password, ('deviceId') : DEVICEID, ('firebaseToken') : firebaseToken, ('userId'): userId]))

def result_0= slurper.parseText(response_0.getResponseBodyContent())
def errCode_0 = result_0.err
println(errCode_0)
def messageFail_0 = result_0.message
println(messageFail_0)
GlobalVariable.tokenAgent = result_0.token.token
println(result_0.token.token)
//
response = WS.sendRequest(findTestObject('Object Repository/Agent_Postman/Home/Cashin/Request Cashin Transaction',
	[('link') : GlobalVariable.link
	,('tokenAgent') : GlobalVariable.tokenAgent,
	 ('SERVICEID') : SERVICEID,
	 ('SENDERPHONE') : SENDERPHONE,
	 ('SENDERCLIENT') : SENDERCLIENT,
	 ('RECEIVERPHONE') : RECEIVERPHONE,
	 ('RECEIVERCLIENT') : RECEIVERCLIENT,
	 ('AMOUNT'): AMOUNT,
	 ('CURRENCY'): CURRENCY,
	 ('MESSAGE'):MESSAGE,
	 ('MessageType'): MessageType,
	  ('DEVICEID'): DEVICEID,
	  ('DEPOSITORPHONE'):DEPOSITORPHONE,
	  ('DEPOSITORNRIC'):DEPOSITORNRIC,
	  ('DEPOSITORNAME'): DEPOSITORNAME
	]))

def result= slurper.parseText(response.getResponseBodyContent())
def errCode = result.err
println(errCode)
GlobalVariable.TRANSREFID=result.data.TRANSREFID
println(result.data.TRANSREFID)
//
response_1 = WS.sendRequest(findTestObject('Object Repository/Agent_Postman/Home/Cashin/Confirm Cashin Transaction',
	[('link') : GlobalVariable.link
	,('tokenAgent') : GlobalVariable.tokenAgent,
	 ('TRANSREFID') : GlobalVariable.TRANSREFID,
	 ('MessageType') : MessageType,
	 ('DEVICEID') : DEVICEID
	]))

def result_1= slurper.parseText(response_1.getResponseBodyContent())
def errCode_1 = result_1.err
println(errCode_1)
def messageFail1 = result_1.message
println(messageFail1)

response_6 = WS.sendRequest(findTestObject('Object Repository/Agent_Postman/Home/Cashin/Verify Cashin Transaction',
	[('link') : GlobalVariable.link,
	('tokenCus') : GlobalVariable.tokenCus,
	('TRANSREFID') : GlobalVariable.TRANSREFID,
	 ('MessageType') : MessageType,
	('PIN') : PIN,
	('OTP'): OTP]))

def result6= slurper.parseText(response_6.getResponseBodyContent())

def errCode6 = result6.err
println(errCode6)
def messageFail6 = result6.message
println(messageFail6)