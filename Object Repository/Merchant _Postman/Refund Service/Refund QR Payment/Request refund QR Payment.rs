<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Request refund QR Payment</name>
   <tag></tag>
   <elementGuidId>52b6fd49-43ec-4fe4-8f02-b15209d74f87</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;SERVICEID&quot;,
      &quot;value&quot;: &quot;5ee0bc1cba0590a90e442b67&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;SENDERPHONE&quot;,
      &quot;value&quot;: &quot;${SENDERPHONE}&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;SENDERCLIENT&quot;,
      &quot;value&quot;: &quot;${SENDERCLIENT}&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;AMOUNT&quot;,
      &quot;value&quot;: &quot;${AMOUNT}&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;CURRENCY&quot;,
      &quot;value&quot;: &quot;${CURRENCY}&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;MESSAGE&quot;,
      &quot;value&quot;: &quot;${MESSAGE}&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;MessageType&quot;,
      &quot;value&quot;: &quot;FO&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;DEVICEID&quot;,
      &quot;value&quot;: &quot;${DEVICEID}&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;RECEIVERPHONE&quot;,
      &quot;value&quot;: &quot;${RECEIVERPHONE}&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;RECEIVERCLIENT&quot;,
      &quot;value&quot;: &quot;${RECEIVERCLIENT}&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;OLDTRANSREFID&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;SUBUSERPHONE&quot;,
      &quot;value&quot;: &quot;${SUBUSERPHONE}&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;SUBUSERCLIENT&quot;,
      &quot;value&quot;: &quot;${SUBUSERCLIENT}&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;ORIGINTRANSREFID&quot;,
      &quot;value&quot;: &quot;${ORIGINTRANSREFID}&quot;,
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
      <value>Bearer ${tokenMerchant}</value>
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
   <restUrl>${link}api/transaction/requestTransaction</restUrl>
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
      <id>570ecf63-2389-475f-9ec5-35637e3cc233</id>
      <masked>false</masked>
      <name>link</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.tokenMerchant</defaultValue>
      <description></description>
      <id>0343bb08-9240-4845-b11c-b8270c2dc52a</id>
      <masked>false</masked>
      <name>tokenMerchant</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>43a0b179-25ff-4140-928b-1f5d816106ee</id>
      <masked>false</masked>
      <name>SENDERPHONE</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>20478093-b4a9-4d8a-95ac-167560d84613</id>
      <masked>false</masked>
      <name>SENDERCLIENT</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>9d0302cc-cf8a-453e-bd5d-8b97647b04cf</id>
      <masked>false</masked>
      <name>AMOUNT</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>a8c184c3-e626-476b-a232-55008cff9997</id>
      <masked>false</masked>
      <name>CURRENCY</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>25d7c0e0-0bcf-4d83-999a-1d629ecfedd3</id>
      <masked>false</masked>
      <name>MESSAGE</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>fcd1af00-77e7-46d5-b79c-eb949118461b</id>
      <masked>false</masked>
      <name>DEVICEID</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>1b96062c-53d5-482a-974f-6c501f86915b</id>
      <masked>false</masked>
      <name>RECEIVERPHONE</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>759390b7-ebf2-482d-a71e-c4dcf1a95f8b</id>
      <masked>false</masked>
      <name>RECEIVERCLIENT</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>03157cee-74b9-46d4-97be-5c1bd0f022ac</id>
      <masked>false</masked>
      <name>SUBUSERPHONE</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>e9877bcc-b8f9-4cb5-b447-ab0ef3a74800</id>
      <masked>false</masked>
      <name>SUBUSERCLIENT</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.transRefId</defaultValue>
      <description></description>
      <id>4e1e13f8-1ef9-44b7-9593-6c79a36fd525</id>
      <masked>false</masked>
      <name>ORIGINTRANSREFID</name>
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
