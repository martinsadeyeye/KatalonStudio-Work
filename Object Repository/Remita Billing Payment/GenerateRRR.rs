<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Generate RRR from a specific Biller/Service type</description>
   <name>GenerateRRR</name>
   <tag></tag>
   <elementGuidId>5d2ddea5-64ba-417e-989f-5668808f8a63</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\&quot;customFields\&quot;: [{\n      \&quot;id\&quot;: \&quot;${customfield}\&quot;,\n      \&quot;values\&quot;: [{\n         \&quot;value\&quot;: \&quot;41961855\&quot;,\n         \&quot;quantity\&quot;: 1,\n         \&quot;amount\&quot;: 10000\n      }]}\n      ],\n      \n        \&quot;billId\&quot;: \&quot;${selectedserviceType}\&quot;,\n   \&quot;amount\&quot;: 10000,\n   \&quot;payerPhone\&quot;: \&quot;08037609808\&quot;,\n   \&quot;currency\&quot;: \&quot;NGN\&quot;,\n   \&quot;payerName\&quot;: \&quot;Martins Adeyeye\&quot;,\n   \&quot;payerEmail\&quot;: \&quot;adeyeye079@gmail.com\&quot;\n}&quot;,
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
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>transactionId</name>
      <type>Main</type>
      <value>6000</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}/generate</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.g_base_url</defaultValue>
      <description></description>
      <id>ae5954e0-ff74-4e1e-be21-7c7c42e9ebd3</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.customerfiled_ID</defaultValue>
      <description></description>
      <id>0bdc1c0f-9ed0-4a29-840b-9c44c0952aea</id>
      <masked>false</masked>
      <name>customfield</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.ServiceTypeID</defaultValue>
      <description></description>
      <id>2f2ca65a-e229-4e81-b7ca-eb9af41f8806</id>
      <masked>false</masked>
      <name>selectedserviceType</name>
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

WS.verifyElementPropertyValue(response, 'responseData[0].amountDue', &quot;10157.5&quot;)
WS.verifyElementPropertyValue(response, 'responseCode', &quot;00&quot;)
WS.verifyElementPropertyValue(response, 'responseMsg', &quot;Successful&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
