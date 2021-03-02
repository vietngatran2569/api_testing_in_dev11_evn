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
//
response_1 = WS.sendRequest(findTestObject('Sub_Postman/Home/Request Transfer/CustomerConfirmRequestTransfer',
	[('link') : GlobalVariable.link
	,('tokenCus') : GlobalVariable.tokenCus,
	 ('phone') : phone,
	 ('amount') : amount,
	 ('currency') : currency,
	 ('message') : message,
	]))

def result_1= slurper.parseText(response_1.getResponseBodyContent())
def errCode_1 = result_1.err
println(errCode_1)
def messageFail1 = result_1.message
println(messageFail1)

//get notification
response_2 = WS.sendRequest(findTestObject('Sub_Postman/1. Login/Login/2.1 Login', [('link') : GlobalVariable.link, ('phone') : phoneLogin2
	, ('password') : password2, ('deviceId') : deviceId2, ('firebaseToken') : firebaseToken2]))
def result_2= slurper.parseText(response_2.getResponseBodyContent())
def errCode_2 = result_2.err
println(errCode_2)
def messageFail_2 = result_2.message
println(messageFail_2)
GlobalVariable.tokenCus = result_2.token.token
println(result_2.token.token)

response_3 = WS.sendRequest(findTestObject('Sub_Postman/Home/Notifications/listNotification', [('link') : GlobalVariable.link,('tokenCus') : GlobalVariable.tokenCus,
	('start'): start, ('number'): number ]))
def result_3= slurper.parseText(response_3.getResponseBodyContent())
//GlobalVariable.requestId = result_3.getAt("data[0]").getAt("cmd").getAt("requestId").toString()
GlobalVariable.requestId = result_3.data[0].cmd.requestId
println(GlobalVariable.requestId)

//REQUEST 
response4 = WS.sendRequest(findTestObject('Sub_Postman/Home/Transfer/transfer from wallet to wallet/Transfer RequestTransaction',
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
response_5 = WS.sendRequest(findTestObject('Sub_Postman/Home/Transfer/transfer from wallet to wallet/Transfer ConfirmTransaction',
	[('link') : GlobalVariable.link,
	('tokenCus') : GlobalVariable.tokenCus,
	('TRANSREFID') : GlobalVariable.TRANSREFID,
	 ('SERVICEID') : SERVICEID,
	 ('MessageType') : MessageType,
	('DEVICEID') : DEVICEID]))
def result5= slurper.parseText(response_5.getResponseBodyContent())
def errCode5 = result5.err
println(errCode5)
def messageFail5 = result5.message
println(messageFail5)
//VERIFY
response_6 = WS.sendRequest(findTestObject('Sub_Postman/Home/Transfer/transfer from wallet to wallet/Transfer VerifyTransaction',
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
