<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>getLocation</name>
   <tag></tag>
   <elementGuidId>1cb4514a-d5bc-402e-9969-407f94f92e8e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;zoom&quot;,
      &quot;value&quot;: &quot;9&quot;
    },
    {
      &quot;name&quot;: &quot;westLng&quot;,
      &quot;value&quot;: &quot;-253.12500000000003&quot;
    },
    {
      &quot;name&quot;: &quot;southLat&quot;,
      &quot;value&quot;: &quot;-80.81689088640861&quot;
    },
    {
      &quot;name&quot;: &quot;eastLng&quot;,
      &quot;value&quot;: &quot;253.12500000000003&quot;
    },
    {
      &quot;name&quot;: &quot;northLat&quot;,
      &quot;value&quot;: &quot;80.76061470752454&quot;
    },
    {
      &quot;name&quot;: &quot;type&quot;,
      &quot;value&quot;: &quot;branch&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>x-www-form-urlencoded</httpBodyType>
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
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${tokenCus}</value>
   </httpHeaderProperties>
   <katalonVersion>7.9.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${link}api/location/getLocation</restUrl>
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
      <id>f0ad6344-f7f9-4b27-8248-37ee3cd1b4d3</id>
      <masked>false</masked>
      <name>link</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.tokenCus</defaultValue>
      <description></description>
      <id>c9253053-5d24-417e-817f-f6a6a37af8a9</id>
      <masked>false</masked>
      <name>tokenCus</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
