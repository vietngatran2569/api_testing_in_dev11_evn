<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>redeem Voucher (backend)</name>
   <tag></tag>
   <elementGuidId>053cf415-fd25-4fb6-bc25-3aafb36ec93d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;voucherCode&quot;,
      &quot;value&quot;: &quot;0075926966807495&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;partnerTransRefId&quot;,
      &quot;value&quot;: &quot;5ec138173f696f1f583a0389&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;partnerCustomerId&quot;,
      &quot;value&quot;: &quot;5ebf6055f644d041be5dd19e&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;partnerMerchantId&quot;,
      &quot;value&quot;: &quot;5ea6935c2a04e65f8888f677&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    },
    {
      &quot;name&quot;: &quot;partnerShopId&quot;,
      &quot;value&quot;: &quot;5ec0bf7e1aa291ab0ec81a64&quot;,
      &quot;type&quot;: &quot;text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>form-data</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>apikey</name>
      <type>Main</type>
      <value>156b6bf9-4a6b-476e-beb8-02fd6c279d47</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept-Language</name>
      <type>Main</type>
      <value>en</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>appid</name>
      <type>Main</type>
      <value>VQGYYG9FHQDOAMR15CVSPOS62GR44L</value>
   </httpHeaderProperties>
   <katalonVersion>7.9.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${loyaltyurl}oapi/customer/applyVoucherCode</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.loyaltyurl</defaultValue>
      <description></description>
      <id>68edcad8-1167-4d6d-b522-7715559ecfd7</id>
      <masked>false</masked>
      <name>loyaltyurl</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
