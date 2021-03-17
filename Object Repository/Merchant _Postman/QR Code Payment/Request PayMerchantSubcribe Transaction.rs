<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Request PayMerchantSubcribe Transaction</name>
   <tag></tag>
   <elementGuidId>1077a469-f2b0-451e-8d16-636e9c9f6127</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;multipart/form-data&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;SENDERPHONE&quot;,
      &quot;value&quot;: &quot;${SENDERPHONE}&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;SENDERCLIENT&quot;,
      &quot;value&quot;: &quot;${SENDERCLIENT}&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;RECEIVERPHONE&quot;,
      &quot;value&quot;: &quot;${RECEIVERPHONE}&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;RECEIVERCLIENT&quot;,
      &quot;value&quot;: &quot;${RECEIVERCLIENT}&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;AMOUNT&quot;,
      &quot;value&quot;: &quot;${AMOUNT}&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;CURRENCY&quot;,
      &quot;value&quot;: &quot;${CURRENCY}&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;MESSAGE&quot;,
      &quot;value&quot;: &quot;${MESSAGE}&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;MessageType&quot;,
      &quot;value&quot;: &quot;${MessageType}&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;DEVICEID&quot;,
      &quot;value&quot;: &quot;${DEVICEID}&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;SUBUSERPHONE&quot;,
      &quot;value&quot;: &quot;${SUBUSERPHONE}&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;SUBUSERCLIENT&quot;,
      &quot;value&quot;: &quot;${SUBUSERCLIENT}&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;QR&quot;,
      &quot;value&quot;: &quot;${QR}&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;SHOPID&quot;,
      &quot;value&quot;: &quot;${SHOPID}&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;VOUCHER&quot;,
      &quot;value&quot;: &quot;${VOUCHER}&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;SERVICEID&quot;,
      &quot;value&quot;: &quot;${SERVICEID}&quot;,
      &quot;type&quot;: &quot;Text&quot;,
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
      <value>multipart/form-data</value>
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
      <id>a59c2b6b-5d35-410e-828e-8a647daa5b9d</id>
      <masked>false</masked>
      <name>link</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.tokenMerchant</defaultValue>
      <description></description>
      <id>25969f77-a31f-40e4-82ed-beac4314acbf</id>
      <masked>false</masked>
      <name>tokenMerchant</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>b7eb6a74-2e2d-44e2-b4f0-964631e5d4ec</id>
      <masked>false</masked>
      <name>SENDERPHONE</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>c8130b86-9e42-4b8c-996c-266cd82b2b68</id>
      <masked>false</masked>
      <name>SENDERCLIENT</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>82235938-40f4-4244-a3fe-8d23d31a203b</id>
      <masked>false</masked>
      <name>RECEIVERPHONE</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>ae2181a9-5367-44ad-8115-e4c80c777292</id>
      <masked>false</masked>
      <name>RECEIVERCLIENT</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>de72e380-fb65-485b-82a1-7098161e7339</id>
      <masked>false</masked>
      <name>AMOUNT</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>58189de2-0895-44c6-86ea-4fcf24bc4c6b</id>
      <masked>false</masked>
      <name>CURRENCY</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>cd9282be-2b74-43b2-8eb4-e7565fa2ab80</id>
      <masked>false</masked>
      <name>MESSAGE</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>759fd9e6-1746-4c34-a1ee-239cb3d916ba</id>
      <masked>false</masked>
      <name>MessageType</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>6d06cd62-ec01-4d5d-a796-76a798329a86</id>
      <masked>false</masked>
      <name>DEVICEID</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>2baad42c-f9b3-484a-b99f-0de2f6925c40</id>
      <masked>false</masked>
      <name>SUBUSERPHONE</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>fe41e310-d2d1-4879-96bc-1fd12efc61c5</id>
      <masked>false</masked>
      <name>SUBUSERCLIENT</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.QRforPaymerchantVoucher</defaultValue>
      <description></description>
      <id>1b49ce20-1e44-41fa-b56e-de63bc139e60</id>
      <masked>false</masked>
      <name>QR</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.ShopId</defaultValue>
      <description></description>
      <id>98b8618e-bc49-44cf-96ff-4091fabaa230</id>
      <masked>false</masked>
      <name>SHOPID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.VoucherCode</defaultValue>
      <description></description>
      <id>85463e85-ba84-4208-9ca7-37c416d0875f</id>
      <masked>false</masked>
      <name>VOUCHER</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>7e77aed7-b187-4e53-80ef-fb399a705004</id>
      <masked>false</masked>
      <name>SERVICEID</name>
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
