<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>ReceiveMoneyRequestTransaction</name>
   <tag></tag>
   <elementGuidId>de351ac8-925b-4133-8a8b-0d478af6dfe5</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;SERVICEID&quot;,
      &quot;value&quot;: &quot;${SERVICEID}&quot;
    },
    {
      &quot;name&quot;: &quot;RECEIVERPHONE&quot;,
      &quot;value&quot;: &quot;${RECEIVERPHONE}&quot;
    },
    {
      &quot;name&quot;: &quot;RECEIVERCLIENT&quot;,
      &quot;value&quot;: &quot;${RECEIVERCLIENT}&quot;
    },
    {
      &quot;name&quot;: &quot;AMOUNT&quot;,
      &quot;value&quot;: &quot;${AMOUNT}&quot;
    },
    {
      &quot;name&quot;: &quot;CURRENCY&quot;,
      &quot;value&quot;: &quot;${CURRENCY}&quot;
    },
    {
      &quot;name&quot;: &quot;MESSAGE&quot;,
      &quot;value&quot;: &quot;${MESSAGE}&quot;
    },
    {
      &quot;name&quot;: &quot;MessageType&quot;,
      &quot;value&quot;: &quot;${MessageType}&quot;
    },
    {
      &quot;name&quot;: &quot;DEVICEID&quot;,
      &quot;value&quot;: &quot;${DEVICEID}&quot;
    },
    {
      &quot;name&quot;: &quot;BENEFICIARYPHONE&quot;,
      &quot;value&quot;: &quot;${BENEFICIARYPHONE}&quot;
    },
    {
      &quot;name&quot;: &quot;DEPOSITORPHONE&quot;,
      &quot;value&quot;: &quot;${DEPOSITORPHONE}&quot;
    },
    {
      &quot;name&quot;: &quot;CASHCODE&quot;,
      &quot;value&quot;: &quot;${CASHCODE}&quot;
    },
    {
      &quot;name&quot;: &quot;TRANSID&quot;,
      &quot;value&quot;: &quot;${TRANSID}&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>x-www-form-urlencoded</httpBodyType>
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
      <id>951fa85e-c052-4368-9c01-0b01f6ce9244</id>
      <masked>false</masked>
      <name>link</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.tokenAgent</defaultValue>
      <description></description>
      <id>71684c68-6d88-443e-b927-e43c72a719bb</id>
      <masked>false</masked>
      <name>tokenAgent</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>b67bf34b-8bcb-40ae-8d55-55354a27c60d</id>
      <masked>false</masked>
      <name>SERVICEID</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>8221b723-a9f9-44cd-b091-ca29c9751950</id>
      <masked>false</masked>
      <name>RECEIVERPHONE</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>a9b75aaf-f917-46a8-9135-d8cf881a7838</id>
      <masked>false</masked>
      <name>RECEIVERCLIENT</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>6c2502b3-39be-4297-9ade-9803ab3c8ef3</id>
      <masked>false</masked>
      <name>AMOUNT</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>d04c4c6d-6421-43f1-81d3-05a4de352ef4</id>
      <masked>false</masked>
      <name>CURRENCY</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>d33f340c-9c82-4504-8777-23a2d00a25d7</id>
      <masked>false</masked>
      <name>MESSAGE</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>bab2750d-62b4-44fd-8a5f-2bc945792a7b</id>
      <masked>false</masked>
      <name>MessageType</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>90e6466b-2bc4-4c00-8b3d-b673c71b2d06</id>
      <masked>false</masked>
      <name>DEVICEID</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>7f414287-cfdb-4b98-a240-2ca737571838</id>
      <masked>false</masked>
      <name>BENEFICIARYPHONE</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>5ba25f97-e671-4e93-8e12-9cced00c31af</id>
      <masked>false</masked>
      <name>DEPOSITORPHONE</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>90d32f9c-1d15-40d3-b526-546c5eff652e</id>
      <masked>false</masked>
      <name>CASHCODE</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>cf56d9ea-85cb-4bf5-a255-19dae0822c4e</id>
      <masked>false</masked>
      <name>TRANSID</name>
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
