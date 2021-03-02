<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>mbl request Copy</name>
   <tag></tag>
   <elementGuidId>d2888da2-bec6-4278-90c9-89be8793e1b8</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n     \&quot;DEVICEID\&quot;: \&quot;6fa831d9f45736e9\&quot;,\r\n            \&quot;MessageType\&quot;: \&quot;FO\&quot;,\r\n            \&quot;MESSAGE\&quot;: \&quot;Abc\&quot;,\r\n            \&quot;CURRENCY\&quot;: \&quot;MMK\&quot;,\r\n            \&quot;RECEIVERCLIENT\&quot;: \&quot;biller\&quot;,\r\n            \&quot;RECEIVERPHONE\&quot;: \&quot;MBLORDERING\&quot;,\r\n            \&quot;SENDERCLIENT\&quot;: \&quot;customer\&quot;,\r\n            \&quot;SENDERPHONE\&quot;: \&quot;09677102111\&quot;,\r\n            \&quot;SERVICEID\&quot;: \&quot;5f6bfe1987f26e2c68bbd794\&quot;,\r\n            \&quot;POSTCODE\&quot;: \&quot;1123092\&quot;,\r\n            \&quot;FIRSTNAME\&quot;: \&quot;My View\&quot;,\r\n            \&quot;PHONE\&quot;: \&quot;091234567\&quot;,\r\n            \&quot;EMAIL\&quot;: \&quot;john.doe@example.com\&quot;,\r\n            \&quot;ADDRRES\&quot;: \&quot;No.294,Than Thumar Rd,(5) Qtr\&quot;,\r\n            \&quot;CITY\&quot;: \&quot;South Okkalapa\&quot;,\r\n            \&quot;PRODUCT_1\&quot;: \&quot;{\\\&quot;id\\\&quot;:\\\&quot;28\\\&quot;,\\\&quot;name\\\&quot;:\\\&quot;BS 30L Drg\\\&quot;,\\\&quot;price\\\&quot;:\\\&quot;76950\\\&quot;,\\\&quot;regular_price\\\&quot;:\\\&quot;76950\\\&quot;    }\&quot;,\r\n            \&quot;QUANTITY_1\&quot;: \&quot;1\&quot;,\r\n            \&quot;deviceId\&quot;: \&quot;6fa831d9f45736e9\&quot;,\r\n            \&quot;url\&quot;: \&quot;/api/transaction/requestTransaction\&quot;,\r\n            \&quot;ip\&quot;: \&quot;103.197.197.16\&quot;\r\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${tokenCus}</value>
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
      <id>9cfc5f45-1e4e-4da6-9e3f-1fe51b4ef114</id>
      <masked>false</masked>
      <name>link</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.tokenCus</defaultValue>
      <description></description>
      <id>d3733b52-f937-47cd-8009-15a5363ff973</id>
      <masked>false</masked>
      <name>tokenCus</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
