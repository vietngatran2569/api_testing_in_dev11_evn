<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Count Transaction by Group Service</name>
   <tag></tag>
   <elementGuidId>ea47a1b4-9ea8-4b98-9cce-0e1377697ca1</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;serviceType&quot;,
      &quot;value&quot;: &quot;5d09b8613a85620ae928711b&quot;
    },
    {
      &quot;name&quot;: &quot;startDate&quot;,
      &quot;value&quot;: &quot;01/01/2019&quot;
    },
    {
      &quot;name&quot;: &quot;endDate&quot;,
      &quot;value&quot;: &quot;09/01/2019&quot;
    },
    {
      &quot;name&quot;: &quot;typeTime&quot;,
      &quot;value&quot;: &quot;searchDate&quot;
    },
    {
      &quot;name&quot;: &quot;typeTime&quot;,
      &quot;value&quot;: &quot;quarter&quot;
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
      <value>application/x-www-form-urlencoded</value>
   </httpHeaderProperties>
   <katalonVersion>7.9.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${link}api/transaction/getTransactionGroupService</restUrl>
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
      <id>51930426-ae62-4424-8d65-6fc03bb49e6a</id>
      <masked>false</masked>
      <name>link</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.tokenAgent</defaultValue>
      <description></description>
      <id>fc44c231-1e3c-494c-a3f8-9fcd7a3e3687</id>
      <masked>false</masked>
      <name>tokenAgent</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
