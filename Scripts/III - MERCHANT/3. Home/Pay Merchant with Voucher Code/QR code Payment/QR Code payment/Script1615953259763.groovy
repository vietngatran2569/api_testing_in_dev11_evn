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

//customer login
response = WS.sendRequest(findTestObject('Sub_Postman/1. Login/Login/2.1 Login', [('link') : GlobalVariable.link, ('phone') : phoneSub
	, ('password') : passwordSub, ('deviceId') : deviceId, ('firebaseToken') : firebaseToken]))

def slurper = new groovy.json.JsonSlurper()

def result = slurper.parseText(response.getResponseBodyContent())

def errCode = result.err

GlobalVariable.tokenCus = result.token.token

println(errCode)

WS.verifyElementPropertyValue(response, 'err', Expected_Err)

WS.sendRequest(findTestObject('Sub_Postman/Profile/getProfile', [('link') : GlobalVariable.link, ('tokenCus') : GlobalVariable.tokenCus]))

//merchant login 
response1 = WS.sendRequest(findTestObject('Object Repository/Merchant _Postman/New Folder/Login/loginWithId', [('link') : GlobalVariable.link, ('phone') : phoneMerchant
	, ('password') : passwordMerchant, ('deviceId') : deviceIdMerchant, ('firebaseToken') : firebaseToken, ('userId'): userId]))

def result1 = slurper.parseText(response1.getResponseBodyContent())

def errCode1 = result1.err

GlobalVariable.tokenMerchant = result1.token.token

println(errCode1)
//merchant get profile to get  merchantId
response2 = WS.sendRequest(findTestObject('Object Repository/Merchant _Postman/Profile/API get profile', [('link') : GlobalVariable.link, ('tokenMerchant') : GlobalVariable.tokenMerchant]))

def result2= slurper.parseText(response2.getResponseBodyContent())

GlobalVariable.merchantId = result2.data.id

println(result2.data.id)

// subcriber get list Promotion
response3 = WS.sendRequest(findTestObject('Object Repository/Ayapoint_Postman/listPromotion', 
	[('link') : GlobalVariable.link, 
	 ('tokenMerchant') : GlobalVariable.tokenMerchant,
	 ('merchantId'): GlobalVariable.merchantId
	 ]))
def result3= slurper.parseText(response3.getResponseBodyContent())
def errCode3 = result3.err
println(errCode3)
GlobalVariable.PromotionId = result3.data[0].sid
println(result3.data.sid)

//subcriber buy promotion
response4 = WS.sendRequest(findTestObject('Object Repository/Ayapoint_Postman/Buy Voucher',
	[('link') : GlobalVariable.link,
	 ('tokenCus') : GlobalVariable.tokenCus,
	 ('promotionId'): GlobalVariable.PromotionId
	 ]))
def result4= slurper.parseText(response4.getResponseBodyContent())
def errCode4 = result4.err
println(errCode4)

//sub get my voucher
response6 = WS.sendRequest(findTestObject('Object Repository/Ayapoint_Postman/myVoucher',
	[('link') : GlobalVariable.link,
	 ('tokenCus') : GlobalVariable.tokenCus
	 ]))
def result6= slurper.parseText(response6.getResponseBodyContent())
def errCode6 = result6.err
GlobalVariable.VoucherCode= result6.data[0].sid
println(errCode6)
println(result6.data[0].sid)

//subcriber scan QR code with voucher(payment qr code)
response5 = WS.sendRequest(findTestObject('Object Repository/Sub_Postman/Home/QR/QR for Paymerchant Voucher',
	[('link') : GlobalVariable.link,
	 ('tokenCus') : GlobalVariable.tokenCus,
	 ('voucherCode'): GlobalVariable.VoucherCode
	 ]))
def result5= slurper.parseText(response5.getResponseBodyContent())
def errCode5 = result5.err
println(errCode5)
GlobalVariable.QRforPaymerchantVoucher = result5.data.qr
println(result5.data.qr)


//merchant get listshop to get shop id
response10 = WS.sendRequest(findTestObject('Object Repository/Merchant _Postman/Shop/getListShop',
	[('link') : GlobalVariable.link,
	('tokenCus') : GlobalVariable.tokenCus,
	('TRANSREFID') : GlobalVariable.TRANSREFID]))
 def result10= slurper.parseText(response10.getResponseBodyContent())
 def errCode10 = result10.err
 println(errCode10)
 GlobalVariable.ShopId=result10.data[0].id

//merchant qr code payment
response7 = WS.sendRequest(findTestObject('Object Repository/Merchant _Postman/QR Code Payment/Request PayMerchantSubcribe Transaction',
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
	('DEVICEID'):deviceIdMerchant,
	('SUBUSERPHONE'):SUBUSERPHONE,
	('SUBUSERCLIENT'):SUBUSERCLIENT,
	('QR'):GlobalVariable.QRforPaymerchantVoucher,
	('SHOPID'):GlobalVariable.ShopId,
	('VOUCHER'):VOUCHER
	]))
def result7= slurper.parseText(response7.getResponseBodyContent())
def errCode7 = result7.err
println(errCode7)
GlobalVariable.TRANSREFID=result7.data.TRANSREFID
println(result7.data.TRANSREFID)
// CONFIRM
response8 = WS.sendRequest(findTestObject('Object Repository/Merchant _Postman/QR Code Payment/Confirm Transaction PayMerchantSubcribe',
   [('link') : GlobalVariable.link,
   ('tokenCus') : GlobalVariable.tokenCus,
   ('TRANSREFID') : GlobalVariable.TRANSREFID,
	('MessageType') : MessageType,
   ('DEVICEID') : deviceIdMerchant]))
def result8= slurper.parseText(response8.getResponseBodyContent())
def errCode8 = result8.err
println(errCode8)

//VERIFY
response9 = WS.sendRequest(findTestObject('Object Repository/Merchant _Postman/QR Code Payment/VerifyTransaction PayMerchantSubcribe',
   [('link') : GlobalVariable.link,
   ('tokenCus') : GlobalVariable.tokenCus,
   ('TRANSREFID') : GlobalVariable.TRANSREFID,
	('MessageType') : MessageType,
   ('PIN') : PIN,
   ('OTP'): OTP]))
def result9= slurper.parseText(response9.getResponseBodyContent())
def errCode9 = result9.err
println(errCode9)






