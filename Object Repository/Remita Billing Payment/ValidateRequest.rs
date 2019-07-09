<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>ValidateRequest</description>
   <name>ValidateRequest</name>
   <tag></tag>
   <elementGuidId>3ca82010-116f-4391-be63-31d1c74dd2de</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\&quot;customFields\&quot;: [{\n      \&quot;id\&quot;: \&quot;${customfieldid}\&quot;,\n      \&quot;values\&quot;: [{\n         \&quot;value\&quot;: \&quot;41961855\&quot;,\n         \&quot;quantity\&quot;: 1,\n         \&quot;amount\&quot;: 10000\n      }]}\n      ],\n      \n        \&quot;billId\&quot;: \&quot;${servicetype}\&quot;,\n   \&quot;amount\&quot;: 10000,\n   \&quot;payerPhone\&quot;: \&quot;08037609808\&quot;,\n   \&quot;currency\&quot;: \&quot;NGN\&quot;,\n   \&quot;payerName\&quot;: \&quot;Martins Adeyeye\&quot;,\n   \&quot;payerEmail\&quot;: \&quot;adeyeye079@gmail.com\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
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
      <value>QzAwMDAxMTU0MDF8MTUwOTM3NzUwMjMzNXw2MGFmMDZjYTk4ZWYwNzgyMjIzMDQ5MTY4MmZhMWYwODFlMTAwODg3NDczMzRkYjFjNWQ5MGMzZmM5ZDQwNDEyMmQ1ZThhZjAwM2YyMmU5ZDA1ZjZkM2QyNTg3OWYyZDFhMDRlYjE4NDM3MjVhODYwOGYxMjdhYmJmNzRkYmQwMA==</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}/validate</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.g_base_url</defaultValue>
      <description></description>
      <id>fb409469-f5bf-4404-a98e-bcfd24beb029</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.ServiceTypeID</defaultValue>
      <description></description>
      <id>6c7f8e82-4566-48b9-a5d9-ebe2faa4288d</id>
      <masked>false</masked>
      <name>servicetype</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.customerfiled_ID</defaultValue>
      <description></description>
      <id>1e90b3f3-d5c3-40a5-b988-8caaad387e3b</id>
      <masked>false</masked>
      <name>customfieldid</name>
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

WS.verifyElementPropertyValue(response, 'responseMsg', &quot;Successful&quot;)
WS.verifyElementPropertyValue(response, 'responseCode', &quot;00&quot;)
WS.verifyElementPropertyValue(response, 'responseData[0].customFields[0].id', &quot;1509377912186&quot;)
WS.verifyElementPropertyValue(response, 'responseData[0].customFields[0].values[0].quantity', 1)
WS.verifyElementPropertyValue(response, 'responseData[0].customFields[0].values[0].amount', 10000.0)
WS.verifyElementPropertyValue(response, 'responseData[0].billId', &quot;1509377912187&quot;)
WS.verifyElementPropertyValue(response, 'responseData[0].payerName', &quot;Martins Adeyeye&quot;)
WS.verifyElementPropertyValue(response, 'responseData[0].amountDue', 10157.5)
WS.verifyElementPropertyValue(response, 'responseData[0].status', &quot;REQUEST_OK&quot;)
WS.verifyElementPropertyValue(response, 'responseData[0].currency', &quot;NGN&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
