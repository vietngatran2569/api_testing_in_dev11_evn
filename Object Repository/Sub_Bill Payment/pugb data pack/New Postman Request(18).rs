<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>New Postman Request(18)</name>
   <tag></tag>
   <elementGuidId>ba0ac058-5ef5-4814-b443-743fc6df98ef</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\r\n    \&quot;serviceId\&quot;: \&quot;5f04048bcbadc1059c5c15cd\&quot;,\r\n            \&quot;billerFormId\&quot;: \&quot;5f040a7fcbadc1059c5c15d3\&quot;,\r\n            \&quot;BILLID\&quot;: \&quot;banggf\&quot;,\r\n            \&quot;METERNO\&quot;: \&quot;BANG\&quot;,\r\n            \&quot;TYPESERVICEIDEM\&quot;: {\r\n                \&quot;name\&quot;: \&quot;YESC\&quot;,\r\n                \&quot;SERVICEIDEM\&quot;: \&quot;NESC-EM\&quot;,\r\n                \&quot;SERVICEID\&quot;: \&quot;5ed8800b1c7eb05ac0c8be79\&quot;,\r\n                \&quot;RECEIVERPHONE\&quot;: \&quot;YESCEM\&quot;,\r\n                \&quot;billerFormId\&quot;: \&quot;5ed885821c7eb05ac0c8be80\&quot;\r\n            },\r\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
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
      <value>application/x-www-form-urlencoded</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>conte</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <katalonVersion>7.9.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${link}api/biller/getListPackages</restUrl>
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
      <id>4b965ba9-c4f3-4ecf-98c8-6496714a2f2e</id>
      <masked>false</masked>
      <name>link</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.tokenCus</defaultValue>
      <description></description>
      <id>297610c2-9e07-499d-819a-f97a9939ce9d</id>
      <masked>false</masked>
      <name>tokenCus</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
