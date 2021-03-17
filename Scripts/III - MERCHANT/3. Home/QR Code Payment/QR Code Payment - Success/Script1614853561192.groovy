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

response = WS.sendRequest(findTestObject('Object Repository/Merchant _Postman/New Folder/Login/loginWithId', [('link') : GlobalVariable.link, ('phone') : phone
	, ('password') : password, ('deviceId') : deviceId, ('firebaseToken') : firebaseToken, ('userId'): userId]))

def slurper = new groovy.json.JsonSlurper()

def result = slurper.parseText(response.getResponseBodyContent())

def errCode = result.err

GlobalVariable.tokenMerchant = result.token.token

println(result.token.token)

WS.verifyElementPropertyValue(response, 'err', Expected_Err)

WS.sendRequest(findTestObject('Object Repository/Merchant _Postman/Profile/API get profile', [('link') : GlobalVariable.link, ('tokenMerchant') : GlobalVariable.tokenMerchant]))

response1 = WS.sendRequest(findTestObject('Object Repository/Merchant _Postman/QR Code Payment/Request PayMerchantSubcribe Transaction',
	[('link') : GlobalVariable.link,
	('tokenCus') : GlobalVariable.tokenCus,
	('SERVICEID') : SERVICEID,
	('SENDERPHONE') : SENDERPHONE,
	('SENDERCLIENT') : SENDERCLIENT,
	('RECEIVERPHONE'): RECEIVERPHONE,
	('RECEIVERCLIENT'):RECEIVERCLIENT,
	('AMOUNT'):AMOUNT,
	('CURRENCY'):CURRENCY,
	('MESSAGE'):MESSAGE,
	('MessageType'):MessageType,
	('DEVICEID'):DEVICEID,
	('SUBUSERPHONE'):SUBUSERPHONE,
	('SUBUSERCLIENT'):SUBUSERCLIENT,
	('QR'):QR,
	('SHOPID'):SHOPID,
	('VOUCHER'):VOUCHER
	]))
def result1= slurper.parseText(response1.getResponseBodyContent())
def errCode_0 = result1.err
println(errCode_0)
def messageFail1 = result1.message
println(messageFail1)
GlobalVariable.TRANSREFID=result1.data.TRANSREFID

// CONFIRM
response2 = WS.sendRequest(findTestObject('Object Repository/Merchant _Postman/QR Code Payment/Confirm Transaction PayMerchantSubcribe',
   [('link') : GlobalVariable.link,
   ('tokenCus') : GlobalVariable.tokenCus,
   ('TRANSREFID') : GlobalVariable.TRANSREFID,
	('MessageType') : MessageType,
   ('DEVICEID') : DEVICEID]))
def result2= slurper.parseText(response2.getResponseBodyContent())
def errCode2 = result2.err
println(errCode2)
def messageFail2 = result2.message
println(messageFail2)

//VERIFY
response3 = WS.sendRequest(findTestObject('Object Repository/Merchant _Postman/QR Code Payment/VerifyTransaction PayMerchantSubcribe',
   [('link') : GlobalVariable.link,
   ('tokenCus') : GlobalVariable.tokenCus,
   ('TRANSREFID') : GlobalVariable.TRANSREFID,
	('MessageType') : MessageType,
   ('PIN') : PIN,
   ('OTP'): OTP]))
def result3= slurper.parseText(response3.getResponseBodyContent())
def errCode3 = result3.err
println(errCode3)
def messageFail3 = result3.message
println(messageFail3)


