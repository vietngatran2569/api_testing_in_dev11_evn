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
response1 = WS.sendRequest(findTestObject('Object Repository/Agent_Postman/Home/Send Money/Receive Money requestTransaction',
	[('link') : GlobalVariable.link
	,('tokenAgent') : GlobalVariable.tokenAgent,
	('CASHCODE'):CASHCODE,
	('TRANSID'):TRANSID,
	 ('SERVICEID') : SERVICEID,
	 ('RECEIVERPHONE') : RECEIVERPHONE,
	 ('RECEIVERCLIENT') : RECEIVERCLIENT,
	 ('BENEFICIARYNRIC') : BENEFICIARYNRIC,
	('BENEFICIARYPHONE') : BENEFICIARYPHONE,
	('DEPOSITORPHONE'):DEPOSITORPHONE,
	('DEPOSITORNRIC'):DEPOSITORNRIC,
	('AMOUNT') : AMOUNT,
	('CURRENCY') : CURRENCY,
	('MESSAGE') : MESSAGE,
	('MessageType') : MessageType,
	('DEVICEID') : DEVICEID,
	]))

def result1= slurper.parseText(response1.getResponseBodyContent())
def errCode1 = result1.err
println(errCode1)
def messageFail1 = result1.message
println(messageFail1)
GlobalVariable.TRANSREFID=result1.data.TRANSREFID

// CONFIRM
response2 = WS.sendRequest(findTestObject('Object Repository/Agent_Postman/Home/Send Money/Receive Money confirmTransaction',
	[('link') : GlobalVariable.link,
	('tokenAgent') : GlobalVariable.tokenAgent,
	('TRANSREFID') : GlobalVariable.TRANSREFID,
	 ('MessageType') : MessageType,
	('DEVICEID') : DEVICEID]))
def result2= slurper.parseText(response2.getResponseBodyContent())
def errCode2 = result2.err
println(errCode2)
def messageFail2 = result2.message
println(messageFail2)
//VERIFY
response_6 = WS.sendRequest(findTestObject('Object Repository/Agent_Postman/Home/Send Money/Receive Money verifyTransaction',
	[('link') : GlobalVariable.link,
	('tokenAgent') : GlobalVariable.tokenAgent,
	('TRANSREFID') : GlobalVariable.TRANSREFID,
	 ('MessageType') : MessageType,
	('PIN') : PIN,
	('OTP'): OTP]))
def result6= slurper.parseText(response_6.getResponseBodyContent())
def errCode6 = result6.err
println(errCode6)
def messageFail6 = result6.message
println(messageFail6)