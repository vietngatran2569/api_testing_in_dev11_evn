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
	, ('password') : password, ('deviceId') : DEVICEID, ('firebaseToken') : firebaseToken, ('userId'): userId]))

def slurper = new groovy.json.JsonSlurper()

def result = slurper.parseText(response.getResponseBodyContent())

def errCode = result.err
def mess = result.message
println(mess)
GlobalVariable.tokenMerchant = result.token.token
println(errCode)
println(result.token.token)

WS.sendRequest(findTestObject('Object Repository/Merchant _Postman/Profile/API get profile', [('link') : GlobalVariable.link, ('tokenMerchant') : GlobalVariable.tokenMerchant]))

//get list shop to get ShopId
response1 = WS.sendRequest(findTestObject('Object Repository/Merchant _Postman/Shop/getListShop', 
	[('link') : GlobalVariable.link,
	 ('tokenMerchant') : GlobalVariable.tokenMerchant]))
def result1 = slurper.parseText(response1.getResponseBodyContent())
def errCode1 = result1.err
println(errCode1)
GlobalVariable.ShopId = result1.data[0].id

//get processedBy( id of subuser)
response2 = WS.sendRequest(findTestObject('Object Repository/Merchant _Postman/Home/New Transaction History/GET USER', 
	[('link') : GlobalVariable.link,
	 ('tokenMerchant') : GlobalVariable.tokenMerchant]))
def result2 = slurper.parseText(response2.getResponseBodyContent())
def errCode2 = result1.err
println(errCode2)
GlobalVariable.processedBy = result2.data[0].id

//get list transHistory
response3 = WS.sendRequest(findTestObject('Object Repository/Merchant _Postman/Home/New Transaction History/API GET LIST transHistory',
	[('link') : GlobalVariable.link,
	 ('tokenMerchant') : GlobalVariable.tokenMerchant,
	 ('startDate'):startDate,
	 ('endDate'):endDate,
	 ('shopId'):GlobalVariable.ShopId,
	 ('processedBy'):GlobalVariable.processedBy
	 ]))
def result3 = slurper.parseText(response3.getResponseBodyContent())
def errCode3 = result3.err
println(errCode3)
GlobalVariable.transRefId = result3.data[0].transRefId

//request refund qr pay merchant
response4 = WS.sendRequest(findTestObject('Object Repository/Merchant _Postman/Refund Service/Refund QR Payment/Request refund QR Payment',
	[('link') : GlobalVariable.link,
	('tokenMerchant') : GlobalVariable.tokenMerchant,
	('SENDERPHONE') : SENDERPHONE,
	('SENDERCLIENT') : SENDERCLIENT,
	('RECEIVERPHONE'): RECEIVERPHONE,
	('RECEIVERCLIENT'):RECEIVERCLIENT,
	('AMOUNT'):AMOUNT,
	('CURRENCY'):CURRENCY,
	('MESSAGE'):MESSAGE,
	('DEVICEID'):DEVICEID,
	('SUBUSERPHONE'):SUBUSERPHONE,
	('SUBUSERCLIENT'):SUBUSERCLIENT,
	('ORIGINTRANSREFID'):GlobalVariable.transRefId
	]))
def result4= slurper.parseText(response4.getResponseBodyContent())
def errCode4 = result4.err
println(errCode4)
GlobalVariable.TRANSREFID=result4.data.TRANSREFID

// CONFIRM
response5 = WS.sendRequest(findTestObject('Object Repository/Merchant _Postman/Refund Service/Refund QR Payment/Refund QR Pay Merchant ConfirmTransaction',
   [('link') : GlobalVariable.link,
  ('tokenMerchant') : GlobalVariable.tokenMerchant,
   ('TRANSREFID') : GlobalVariable.TRANSREFID,
   ('DEVICEID') : DEVICEID]))
def result5= slurper.parseText(response5.getResponseBodyContent())
def errCode5 = result5.err
println(errCode5)

//VERIFY
response6 = WS.sendRequest(findTestObject('Object Repository/Merchant _Postman/Refund Service/Refund QR Payment/Refund QR Pay Merchant verifyTransaction',
   [('link') : GlobalVariable.link,
   ('tokenCus') : GlobalVariable.tokenCus,
   ('TRANSREFID') : GlobalVariable.TRANSREFID,
   ('PIN') : PIN,
   ('OTP'): OTP]))
def result6= slurper.parseText(response6.getResponseBodyContent())
def errCode6 = result6.err
println(errCode6)




