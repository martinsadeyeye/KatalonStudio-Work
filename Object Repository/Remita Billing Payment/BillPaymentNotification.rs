<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>BillPaymentNotification</description>
   <name>BillPaymentNotification</name>
   <tag></tag>
   <elementGuidId>1704a26b-52e6-46fe-b823-6ce3f8e03496</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\&quot;rrr\&quot;:  \&quot;${rrr}\&quot;,\n\&quot;incomeAccount\&quot;: \&quot;${incomeAccount}\&quot;,\n\&quot;debittedAccount\&quot;: \&quot;${debittedAccount}\&quot;, \n\&quot;paymentAuthCode\&quot;: \&quot;${paymentAuthCode}\&quot;,\n\&quot;paymentChannel\&quot;:  \&quot;${paymentChannel}\&quot;,\n\&quot;tellerName\&quot;:  \&quot;${tellerName}\&quot;,\n\&quot;branchCode\&quot;:  \&quot;${branchCode}\&quot;,\n\&quot;amountDebitted\&quot;:\&quot;${amountDebitted}\&quot;, \n\&quot;fundingSource\&quot;: \&quot;${productCode}\&quot;,\n\&quot;hash\&quot;: \&quot;${hash}\&quot;\n}&quot;,
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
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>TXN_HASH</name>
      <type>Main</type>
      <value>${hash}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}/payment/notify</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.g_base_url</defaultValue>
      <description></description>
      <id>bce5d492-b40a-4b1f-ae14-46352419939f</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>'300216918740'</defaultValue>
      <description></description>
      <id>f20c11e9-3420-47ff-875e-15a816c8a9ad</id>
      <masked>false</masked>
      <name>rrr</name>
   </variables>
   <variables>
      <defaultValue>'0012618858'</defaultValue>
      <description></description>
      <id>487d3c7e-fa51-4334-bb01-bd7aec6df0a9</id>
      <masked>false</masked>
      <name>debittedAccount</name>
   </variables>
   <variables>
      <defaultValue>'5tuueiiee_abc23400'</defaultValue>
      <description></description>
      <id>66535cd3-7b6c-424b-85ea-5cc577da560b</id>
      <masked>false</masked>
      <name>paymentAuthCode</name>
   </variables>
   <variables>
      <defaultValue>10000</defaultValue>
      <description></description>
      <id>c7709bf7-6629-43c0-bfda-c44083899826</id>
      <masked>false</masked>
      <name>amountDebitted</name>
   </variables>
   <variables>
      <defaultValue>'GTB'</defaultValue>
      <description></description>
      <id>1f63cbf7-f2ee-4f3e-b4e1-b97278acca15</id>
      <masked>false</masked>
      <name>productCode</name>
   </variables>
   <variables>
      <defaultValue>'012345689'</defaultValue>
      <description></description>
      <id>31a8e59f-91a5-411f-9362-4d5af4a2bee2</id>
      <masked>false</masked>
      <name>incomeAccount</name>
   </variables>
   <variables>
      <defaultValue>'USSD'</defaultValue>
      <description></description>
      <id>37c8949f-7f65-4de6-8ef4-99c6e0de35c4</id>
      <masked>false</masked>
      <name>paymentChannel</name>
   </variables>
   <variables>
      <defaultValue>'Mike Oshadami'</defaultValue>
      <description></description>
      <id>9ff45ebe-615a-403b-a4c8-b7bb593f8ebe</id>
      <masked>false</masked>
      <name>tellerName</name>
   </variables>
   <variables>
      <defaultValue>'BBT563'</defaultValue>
      <description></description>
      <id>76e922c1-7065-4145-8767-7a4c980d789f</id>
      <masked>false</masked>
      <name>branchCode</name>
   </variables>
   <variables>
      <defaultValue>'610a64055d214207ee638c1dd7c610b1751dcc0510563fdc52c0a4f9f8e36e275c686c6cbae1ea4e636131a265f20f07e8d610a4733f4df974c0f915465048a1'</defaultValue>
      <description></description>
      <id>0ad95b52-eda6-4dcb-8db0-13f1a08a0e6d</id>
      <masked>false</masked>
      <name>secretKey</name>
   </variables>
   <variables>
      <defaultValue>'CryptoJS.SHA512( rrr + amountDebitted + productCode + debittedAccount + paymentAuthCode + secretKey)'</defaultValue>
      <description></description>
      <id>c077294c-0071-4cbc-86a3-bbe1050a587f</id>
      <masked>false</masked>
      <name>hash</name>
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
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
