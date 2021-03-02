<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>get customer Copy</name>
   <tag></tag>
   <elementGuidId>4bb6a49e-db36-48a0-9eed-5c816a76469f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\&quot;transRefId\&quot;:\&quot;12345\&quot;,\n \&quot;customerId\&quot;: \&quot;9\&quot;,\n \&quot;amount\&quot;:\&quot;1000\&quot;,\n\&quot;enquiry\&quot;:false,\n    \&quot;first_name\&quot;: \&quot;Jan\&quot;,\n    \&quot;address_1\&quot;: \&quot;27 Park Road\&quot;,\n    \&quot;city\&quot;: \&quot;Yangon\&quot;,\n    \&quot;email\&quot;: \&quot;jan@example.com\&quot;,\n    \&quot;phone\&quot;: \&quot;0912345678\&quot;,\n    \&quot;line_items\&quot;: [\n        {\n            \&quot;product_id\&quot;: 21,\n            \&quot;quantity\&quot;: 1\n        }\n    ]\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
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
   <restUrl>${linkNodered}biller/ayaymbl/order</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.linkNodered</defaultValue>
      <description></description>
      <id>6706dee6-95d5-4762-8f48-83014fd5c6b3</id>
      <masked>false</masked>
      <name>linkNodered</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
