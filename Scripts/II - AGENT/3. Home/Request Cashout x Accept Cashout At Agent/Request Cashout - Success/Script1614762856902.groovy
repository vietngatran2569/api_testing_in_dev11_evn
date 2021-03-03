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
//login agent
response = WS.sendRequest(findTestObject('Object Repository/Agent_Postman/New Folder/Login/Login With UserId', [('link') : GlobalVariable.link, ('phone') : phone
	, ('password') : password, ('deviceId') : deviceId, ('firebaseToken') : firebaseToken, ('userId'): userId]))

def result = slurper.parseText(response.getResponseBodyContent())

def errCode = result.err

GlobalVariable.tokenAgent = result.token.token

println(result.token.token)

WS.verifyElementPropertyValue(response, 'err', Expected_Err)

WS.sendRequest(findTestObject('Object Repository/Agent_Postman/Profile/API get profile for AGENT', [('link') : GlobalVariable.link, ('tokenAgent') : GlobalVariable.tokenAgent]))

//request cashout
response1 = WS.sendRequest(findTestObject('Object Repository/Agent_Postman/Request Cashout/Request CashOut',
	 [('link') : GlobalVariable.link,
     ('tokenAgent') : GlobalVariable.tokenAgent,
	 ('phone') : phoneReceiver,
	 ('name') : nameReceiver, 
	 ('amount') : amount, 
	 ('message') : message, 
	 ('currency'): currency]))
def result1= slurper.parseText(response1.getResponseBodyContent())
def errCode_0 = result1.err
println(errCode_0)
//confirm cashout
response2 = WS.sendRequest(findTestObject('Object Repository/Agent_Postman/Request Cashout/Cash out Confirm',
	 [('link') : GlobalVariable.link,
	 ('tokenAgent') : GlobalVariable.tokenAgent,
	 ('phone') : phoneReceiver,
	 ('name') : nameReceiver,
	 ('amount') : amount,
	 ('message') : message,
	 ('currency'): currency]))
def result2= slurper.parseText(response2.getResponseBodyContent())
def errCode2 = result2.err
println(errCode2)

//login customer
response_0 = WS.sendRequest(findTestObject('Sub_Postman/1. Login/Login/2.1 Login', [('link') : GlobalVariable.link, ('phone') : phoneCus
	, ('password') : passwordCus, ('deviceId') : deviceId, ('firebaseToken') : firebaseToken]))
def result_0= slurper.parseText(response_0.getResponseBodyContent())
def errCode0 = result_0.err
println(errCode0)

GlobalVariable.tokenCus = result_0.token.token
println(result_0.token.token)
//get noti
response_3 = WS.sendRequest(findTestObject('Sub_Postman/Home/Notifications/listNotification', [('link') : GlobalVariable.link,('tokenCus') : GlobalVariable.tokenCus,
	('start'): start, ('number'): number ]))
def result_3= slurper.parseText(response_3.getResponseBodyContent())
//GlobalVariable.requestId = result_3.getAt("data[0]").getAt("cmd").getAt("requestId").toString()
GlobalVariable.requestId = result_3.data[0].cmd.requestId
println(GlobalVariable.requestId)
// request cashout at agent
response4 = WS.sendRequest(findTestObject('Object Repository/Sub_Postman/Home/Cash out/cash out at agent/Request Subcriber confirm cash out at agent',
	[('link') : GlobalVariable.link
	,('tokenCus') : GlobalVariable.tokenCus,
	 ('SERVICEID') : SERVICEID,
	 ('SENDERPHONE') : SENDERPHONE,
	 ('SENDERCLIENT') : SENDERCLIENT,
	 ('RECEIVERPHONE') : RECEIVERPHONE,
	('RECEIVERCLIENT') : RECEIVERCLIENT,
	('AMOUNT') : AMOUNT,
	('CURRENCY') : CURRENCY,
	('MESSAGE') : MESSAGE,
	('MessageType') : MessageType,
	('DEVICEID') : DEVICEID,
	('REQUESTID'): GlobalVariable.requestId]))

def result4= slurper.parseText(response4.getResponseBodyContent())
def errCode4 = result4.err
println(errCode4)
def messageFail4 = result4.message
println(messageFail4)
GlobalVariable.TRANSREFID=result4.data.TRANSREFID

// CONFIRM
response_5 = WS.sendRequest(findTestObject('Object Repository/Sub_Postman/Home/Cash out/cash out at agent/Confrim Subcriber confrim cash out at agent',
	[('link') : GlobalVariable.link,
	('tokenCus') : GlobalVariable.tokenCus,
	('TRANSREFID') : GlobalVariable.TRANSREFID,
	 ('MessageType') : MessageType,
	('DEVICEID') : DEVICEID]))
def result5= slurper.parseText(response_5.getResponseBodyContent())
def errCode5 = result5.err
println(errCode5)
def messageFail5 = result5.message
println(messageFail5)
//VERIFY
response_6 = WS.sendRequest(findTestObject('Object Repository/Sub_Postman/Home/Cash out/cash out at agent/VerifyTransaction subcriber confirm cash out at agent',
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





