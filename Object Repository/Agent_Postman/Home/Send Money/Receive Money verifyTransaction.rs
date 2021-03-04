<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Receive Money verifyTransaction</name>
   <tag></tag>
   <elementGuidId>b8c1638e-7730-45af-b090-ba050130bdce</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;OTP&quot;,
      &quot;value&quot;: &quot;${OTP}&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;MessageType&quot;,
      &quot;value&quot;: &quot;${MessageType}&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;TRANSREFID&quot;,
      &quot;value&quot;: &quot;${TRANSREFID}&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;PIN&quot;,
      &quot;value&quot;: &quot;${PIN}&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>form-data</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${tokenAgent}</value>
   </httpHeaderProperties>
   <katalonVersion>7.9.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${link}api/transaction/verifyTransaction</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.link</defaultValue>
      <description></description>
      <id>33c9e193-4c76-4586-9870-451000225ed4</id>
      <masked>false</masked>
      <name>link</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.tokenAgent</defaultValue>
      <description></description>
      <id>d210e468-0122-4684-a49f-c9391501dad2</id>
      <masked>false</masked>
      <name>tokenAgent</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>214bacf1-a0ac-4080-8aec-a15210fa09e2</id>
      <masked>false</masked>
      <name>OTP</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>52ed1583-289b-4f85-97ad-5d1a986bb7d7</id>
      <masked>false</masked>
      <name>MessageType</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.TRANSREFID1</defaultValue>
      <description></description>
      <id>17c8bc8f-aaed-4aa6-91e5-9fa9aaafb6f2</id>
      <masked>false</masked>
      <name>TRANSREFID</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>e0332b24-a520-4103-88bf-ba0cbabf0b39</id>
      <masked>false</masked>
      <name>PIN</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
