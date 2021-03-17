<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>findLocation NEW</name>
   <tag></tag>
   <elementGuidId>336aa540-0653-4d2b-8217-3fc6c04b89f5</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;long&quot;,
      &quot;value&quot;: &quot;152.444382&quot;
    },
    {
      &quot;name&quot;: &quot;lat&quot;,
      &quot;value&quot;: &quot;-32.451462&quot;
    },
    {
      &quot;name&quot;: &quot;distance&quot;,
      &quot;value&quot;: &quot;500000000000&quot;
    },
    {
      &quot;name&quot;: &quot;search&quot;,
      &quot;value&quot;: &quot;Bawga Theddi&quot;
    },
    {
      &quot;name&quot;: &quot;start&quot;,
      &quot;value&quot;: &quot;0&quot;
    },
    {
      &quot;name&quot;: &quot;number&quot;,
      &quot;value&quot;: &quot;30&quot;
    },
    {
      &quot;name&quot;: &quot;type&quot;,
      &quot;value&quot;: &quot;atm&quot;
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
   <restUrl>${link}api/location/findLocation</restUrl>
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
      <id>ff3fd18d-a2fb-4ce4-a208-780b3c7b53e7</id>
      <masked>false</masked>
      <name>link</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.tokenCus</defaultValue>
      <description></description>
      <id>08feac35-8a05-4f18-9cae-3f358b0c7517</id>
      <masked>false</masked>
      <name>tokenCus</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
