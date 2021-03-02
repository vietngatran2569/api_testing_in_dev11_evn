<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Subcriber viu requestTransaction</name>
   <tag></tag>
   <elementGuidId>40603704-e82d-4d73-889e-543b3f2b3fe9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;SERVICEID\&quot;: \&quot;${SERVICEID}\&quot;,\n    \&quot;SENDERCLIENT\&quot;: \&quot;${SENDERCLIENT}\&quot;,\n    \&quot;RECEIVERPHONE\&quot;: \&quot;${RECEIVERPHONE}\&quot;,\n    \&quot;RECEIVERCLIENT\&quot;: \&quot;${RECEIVERCLIENT}\&quot;,\n    \&quot;SENDERPHONE\&quot;: \&quot;${SENDERPHONE}\&quot;,\n    \&quot;CURRENCY\&quot;: \&quot;${CURRENCY}\&quot;,\n    \&quot;MESSAGE\&quot;: \&quot;${MESSAGE}\&quot;,\n    \&quot;MessageType\&quot;: \&quot;${MessageType}\&quot;,\n    \&quot;DEVICEID\&quot;: \&quot;${DEVICEID}\&quot;,\n    \&quot;NOTE\&quot;: \&quot;${NOTE}\&quot;,\n    \&quot;AMOUNT\&quot;: \&quot;${AMOUNT}\&quot;,\n    \&quot;deviceId\&quot;: \&quot;${deviceId}\&quot;,\n    \&quot;url\&quot;: \&quot;/api/transaction/requestTransaction\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer 07978545-140b-474f-b366-d23a5fb909f4</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
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
      <id>45229d55-a133-49ba-8043-ff95e3afe990</id>
      <masked>false</masked>
      <name>link</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.tokenCus</defaultValue>
      <description></description>
      <id>7d3d01e3-3bb6-4c6a-8fdf-2e8271deb3d1</id>
      <masked>false</masked>
      <name>tokenCus</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>3b17f45a-e87c-451b-a1b1-2f64ac3ac6ed</id>
      <masked>false</masked>
      <name>SERVICEID</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>e995230c-044c-4cb1-90f2-ae36ca6becc5</id>
      <masked>false</masked>
      <name>SENDERCLIENT</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>dd474057-b8b1-46d0-8d64-80272b7cf3d0</id>
      <masked>false</masked>
      <name>RECEIVERPHONE</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>fa22301a-a5e1-4464-a418-5213df209d6a</id>
      <masked>false</masked>
      <name>RECEIVERCLIENT</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>e037c780-32df-45b3-9a2f-db64a02f5b4f</id>
      <masked>false</masked>
      <name>SENDERPHONE</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>5596b799-d6f4-4469-a3b3-c2027ca33849</id>
      <masked>false</masked>
      <name>CURRENCY</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>09cd4248-6bb1-4b66-a41b-30872f7f9067</id>
      <masked>false</masked>
      <name>MESSAGE</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>c742dfd9-4a53-4e47-8a1f-ced9ae29ccbe</id>
      <masked>false</masked>
      <name>MessageType</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>bd3e5b54-9312-4b08-9726-381ac267291f</id>
      <masked>false</masked>
      <name>DEVICEID</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>b0307efc-87b9-4849-8f86-e1eb39a9653c</id>
      <masked>false</masked>
      <name>NOTE</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>f11db691-64a1-49ab-bf13-d3fb734dde2b</id>
      <masked>false</masked>
      <name>AMOUNT</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>f8442230-638b-4dad-a8f1-778b1df681fd</id>
      <masked>false</masked>
      <name>deviceId</name>
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
