<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Request Pay Online Merchant Transaction</name>
   <tag></tag>
   <elementGuidId>ba75a37b-2bf2-492b-9cf2-ec985920c8da</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;SERVICEID&quot;,
      &quot;value&quot;: &quot;5e216cc54d067ef2b7adafb2&quot;
    },
    {
      &quot;name&quot;: &quot;SENDERPHONE&quot;,
      &quot;value&quot;: &quot;09677102111&quot;
    },
    {
      &quot;name&quot;: &quot;SENDERCLIENT&quot;,
      &quot;value&quot;: &quot;customer&quot;
    },
    {
      &quot;name&quot;: &quot;RECEIVERPHONE&quot;,
      &quot;value&quot;: &quot;09684951189&quot;
    },
    {
      &quot;name&quot;: &quot;RECEIVERCLIENT&quot;,
      &quot;value&quot;: &quot;merchant&quot;
    },
    {
      &quot;name&quot;: &quot;AMOUNT&quot;,
      &quot;value&quot;: &quot;{{AMOUNT}}&quot;
    },
    {
      &quot;name&quot;: &quot;CURRENCY&quot;,
      &quot;value&quot;: &quot;{{CURRENCY}}&quot;
    },
    {
      &quot;name&quot;: &quot;MESSAGE&quot;,
      &quot;value&quot;: &quot;{{THIRDPARTYDATA}}&quot;
    },
    {
      &quot;name&quot;: &quot;MessageType&quot;,
      &quot;value&quot;: &quot;FO&quot;
    },
    {
      &quot;name&quot;: &quot;DEVICEID&quot;,
      &quot;value&quot;: &quot;deviceid_for_example&quot;
    },
    {
      &quot;name&quot;: &quot;SUBUSERPHONE&quot;,
      &quot;value&quot;: &quot;{{SUBUSERPHONE}}&quot;
    },
    {
      &quot;name&quot;: &quot;SUBUSERCLIENT&quot;,
      &quot;value&quot;: &quot;{{SUBUSERCLIENT}}&quot;
    },
    {
      &quot;name&quot;: &quot;REQUESTID&quot;,
      &quot;value&quot;: &quot;{{REQUESTID}}&quot;
    },
    {
      &quot;name&quot;: &quot;TRANSACTIONID&quot;,
      &quot;value&quot;: &quot;{{TRANSACTIONID}}&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>x-www-form-urlencoded</httpBodyType>
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
   <katalonVersion>7.8.0</katalonVersion>
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
      <id>9a5a84b9-7f58-4fb2-8525-0993dfa13bd1</id>
      <masked>false</masked>
      <name>link</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.tokenCus</defaultValue>
      <description></description>
      <id>65fe0f00-57d3-4848-aa81-b26384ad17bf</id>
      <masked>false</masked>
      <name>tokenCus</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
