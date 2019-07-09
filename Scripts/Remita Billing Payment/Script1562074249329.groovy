import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

ListOfALLBillers = WS.sendRequestAndVerify(findTestObject('Remita Billing Payment/GetRestrictedBillers', [('url') : GlobalVariable.g_base_url]))

def slurper = new groovy.json.JsonSlurper()

def result = slurper.parseText(ListOfALLBillers.getResponseBodyContent())

def value = result.responseData[4].id

println('********************Corporate ID extracted is*************** : ' + value)

GlobalVariable.BillerID = value

println('********************Global Variable for Biller is now*************** : ' + GlobalVariable.BillerID)


Servicetype_ID = WS.sendRequestAndVerify(findTestObject('Remita Billing Payment/GetService'))

def slurper1 = new groovy.json.JsonSlurper()

def result1 = slurper1.parseText(Servicetype_ID.getResponseBodyContent())

def value1 = (result1.responseData[0])[3].id

println('********************Service type ID extracted is*************** : ' + value1)

GlobalVariable.ServiceTypeID = value1

println('********************Global Variable for Service Type is now*************** : ' + GlobalVariable.ServiceTypeID)


customerfiled_ID = WS.sendRequestAndVerify(findTestObject('Remita Billing Payment/GetCustomFields'))

def slurper10 = new groovy.json.JsonSlurper()

def result10 = slurper10.parseText(customerfiled_ID.getResponseBodyContent())

def value10 = result10.responseData[0].id

println('********************Custom ID extracted is*************** : ' + value10)

GlobalVariable.customerfiled_ID = value10

println('********************Global Variable for Custom ID is now*************** : ' + GlobalVariable.customerfiled_ID)


ValidateReq = WS.sendRequestAndVerify(findTestObject('Remita Billing Payment/ValidateRequest'))

def slurper11 = new groovy.json.JsonSlurper()

def result11 = slurper11.parseText(ValidateReq.getResponseBodyContent())

def value11 = result11.responseData[0].customFields[0].id

println('********************Validated custom field is*************** : ' + value11)

GlobalVariable.ValidateReq = value11

println('********************Global Variable forValidated custom field is now*************** : ' + GlobalVariable.ValidateReq)



RRR = WS.sendRequestAndVerify(findTestObject('Remita Billing Payment/GenerateRRR'))

def slurper2 = new groovy.json.JsonSlurper()

def result2 = slurper2.parseText(RRR.getResponseBodyContent())

def value2 = result2.responseData[0].rrr

println('*******************RRR generated  is*************** : ' + value2)

GlobalVariable.RRR = value2

println('********************Global Variable for RRR is now*************** : ' + GlobalVariable.RRR)

RRR_details = WS.sendRequestAndVerify(findTestObject('Remita Billing Payment/GetRRRDetails'))

def slurper3 = new groovy.json.JsonSlurper()

def result3 = slurper3.parseText(RRR.getResponseBodyContent())

def value3 = result3.responseData[0].amountDue

println('***************RRR amount is  *****************' + value3)

