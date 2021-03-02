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
import groovy.json.JsonSlurper

WS.sendRequest(findTestObject('Sub_Postman/1. Login/Register/1.2 registerVerifyOTP', [('link') : GlobalVariable.link, ('phone') : ''
            , ('otp') : '', ('deviceId') : '']))

response_3 = WS.sendRequest(findTestObject('Sub_Postman/1. Login/Register/1.3 registerInfo', [('link') : GlobalVariable.link
            , ('phone') : phone, ('firebaseToken') : firebaseToken, ('deviceId') : deviceId, ('mpin') : mpin, ('avatar') : avatar
            , ('name') : name, ('gender') : gender, ('birth') : birth]))
def slurper = new groovy.json.JsonSlurper()
def result3 = slurper.parseText(response_3.getResponseBodyContent())

//WS.verifyElementPropertyValue(response_2, 'message', 'Success')

