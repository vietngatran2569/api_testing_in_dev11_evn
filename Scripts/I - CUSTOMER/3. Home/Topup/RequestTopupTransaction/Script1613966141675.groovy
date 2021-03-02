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
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable

response = WS.sendRequest(findTestObject('Sub_Postman/Home/Mobile Topup/Request Topup Transaction',
	[('link') : GlobalVariable.link
	,('tokenCus') : GlobalVariable.tokenCus,
	 ('SERVICEID') : SERVICEID,
	 ('SENDERPHONE') : SENDERPHONE,
	 ('SENDERCLIENT') : SENDERCLIENT,
	('AMOUNT') : AMOUNT,
	('CURRENCY') : CURRENCY,
	('MESSAGE') : MESSAGE,
	('MessageType') : MessageType,
	('DEVICEID') : DEVICEID,
	('BENEFICIARYPHONE') : BENEFICIARYPHONE,
	('RECEIVERPHONE') : RECEIVERPHONE,
	('RECEIVERCLIENT') : RECEIVERCLIENT
	]))

def slurper = new groovy.json.JsonSlurper()
def result= slurper.parseText(response.getResponseText())

def errCode = result.err
println(errCode)

GlobalVariable.TxnID_Bill=result.data.TRANSREFID
//println(result.data.TRANSREFID)

WS.verifyElementPropertyValue(response, "message", "Success")
