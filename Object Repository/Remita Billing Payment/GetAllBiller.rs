<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Get Biller on Remita Platform</description>
   <name>GetAllBiller</name>
   <tag></tag>
   <elementGuidId>cd541458-d48a-4bbd-98b9-7e76b784fe2b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>publicKey</name>
      <type>Main</type>
      <value>U0hFTEx8MTUwOTM3MTg1NDczOXwyNjdjNDBiZmI5ZjMzMjg5M2I3MWI2YzEzZWUxYTQ5YjUxOTRhMjY5ZDljOWUzNmI0MWUxOTgyYzI1NDUyYTMxM2NlM2QxYTdmZjQxMTExN2M5MTU1NjgxNWYyYmEwMTI3ZWY3MmU4M2MxNmE2ZjBmNjE3Y2Q2OTNlYzA1ODA4Nw==</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>transactionId</name>
      <type>Main</type>
      <value>23</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${url}/billers</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.g_base_url</defaultValue>
      <description></description>
      <id>466f2bed-bbbc-4772-bbcc-773570ba2bb8</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()



assertThat(response.getResponseText()).contains('2LEVEL TECHNOLOGIES')
WS.verifyElementPropertyValue(response, 'responseData[273].id', &quot;C0000115401&quot;)
WS.verifyElementPropertyValue(response, 'responseData[273].description', &quot;Prado Lee Nig&quot;)
WS.verifyElementPropertyValue(response, 'responseData[273].label', &quot;Prado Lee Nig&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
