<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Scan QR code</name>
   <tag></tag>
   <elementGuidId>4a3568a7-7c85-4f7c-852a-a9827872c11c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;qrcode&quot;,
      &quot;value&quot;: &quot;00020351245d01c0a9a043e906d267c002&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;qrcode&quot;,
      &quot;value&quot;: &quot;00020251245d00b2cc82de9406ce0b0dd8&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;qrcode&quot;,
      &quot;value&quot;: &quot;000201010211503500245cf5f3deed84401e587690320103123515200245d0c9ac82ae5bf0288e442a50120Y8H6452143Y8H64521435802115910Merchant36006Yangon64200002vn0110Merchant 36304c523&quot;,
      &quot;type&quot;: &quot;text&quot;,
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
      <value>Bearer 06c29fc5-b570-4cb9-8d79-ec6680a3d2f1</value>
   </httpHeaderProperties>
   <katalonVersion>7.9.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${link}/qrcode/getInfo?</restUrl>
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
      <id>2aac665c-2432-4427-a1a4-3b98c4d5778d</id>
      <masked>false</masked>
      <name>link</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
