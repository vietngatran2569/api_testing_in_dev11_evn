<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Receive Money confirmTransaction</name>
   <tag></tag>
   <elementGuidId>a863ca90-1858-496d-b367-a81a73160fae</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;parameters&quot;: [
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
      &quot;name&quot;: &quot;DEVICEID&quot;,
      &quot;value&quot;: &quot;${DEVICEID}&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>form-data</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${tokenAgent}</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
   </httpHeaderProperties>
   <katalonVersion>7.9.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${link}api/transaction/confirmTransaction</restUrl>
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
      <id>c23e901c-b954-4c00-a9e7-e6063ab0d146</id>
      <masked>false</masked>
      <name>link</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.tokenAgent</defaultValue>
      <description></description>
      <id>3db384a8-543a-4799-91bc-3d68cff7fb62</id>
      <masked>false</masked>
      <name>tokenAgent</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>fa885cbe-e1b2-4b5f-b926-1a2ef783134d</id>
      <masked>false</masked>
      <name>MessageType</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.TRANSREFID1</defaultValue>
      <description></description>
      <id>b3c0be24-1bf1-4941-808f-4d9fc5e86201</id>
      <masked>false</masked>
      <name>TRANSREFID</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>6e02319b-dbf9-42fc-81ad-1fc67e88d48f</id>
      <masked>false</masked>
      <name>DEVICEID</name>
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
