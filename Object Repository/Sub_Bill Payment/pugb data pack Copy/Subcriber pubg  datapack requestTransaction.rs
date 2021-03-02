<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Subcriber pubg  datapack requestTransaction</name>
   <tag></tag>
   <elementGuidId>4dd72001-3177-4176-ba99-a6154d64d002</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;SERVICEID&quot;,
      &quot;value&quot;: &quot;5f71878466850168f246b797&quot;
    },
    {
      &quot;name&quot;: &quot;CURRENCY&quot;,
      &quot;value&quot;: &quot;MMK&quot;
    },
    {
      &quot;name&quot;: &quot;DEVICEID&quot;,
      &quot;value&quot;: &quot;6fa831d9f45736e9&quot;
    },
    {
      &quot;name&quot;: &quot;MESSAGE&quot;,
      &quot;value&quot;: &quot;Abc&quot;
    },
    {
      &quot;name&quot;: &quot;MessageType&quot;,
      &quot;value&quot;: &quot;FO&quot;
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
      &quot;name&quot;: &quot;PAYMENTTYPE&quot;,
      &quot;value&quot;: &quot;LoanRepayment&quot;
    },
    {
      &quot;name&quot;: &quot;LOANACCOUNTNUMBER&quot;,
      &quot;value&quot;: &quot;321455&quot;
    },
    {
      &quot;name&quot;: &quot;CHANNELREFID&quot;,
      &quot;value&quot;: &quot;ce813dd2-429c-4660-8f4d-b62a05a0fc4c&quot;
    },
    {
      &quot;name&quot;: &quot;BILLERREFID&quot;,
      &quot;value&quot;: &quot;D2508F35159645D8B2BEDD2443F24098&quot;
    },
    {
      &quot;name&quot;: &quot;CUSTOMERNAME&quot;,
      &quot;value&quot;: &quot;Daw Aye Aye&quot;
    },
    {
      &quot;name&quot;: &quot;CUSTOMERPHONE&quot;,
      &quot;value&quot;: &quot;09960348696&quot;
    },
    {
      &quot;name&quot;: &quot;CUSTOMEREMAIL&quot;,
      &quot;value&quot;: &quot;No email&quot;
    },
    {
      &quot;name&quot;: &quot;LOANOFFICENAME&quot;,
      &quot;value&quot;: &quot;Branch Office - Tatkon&quot;
    },
    {
      &quot;name&quot;: &quot;DUEDATE&quot;,
      &quot;value&quot;: &quot;2020-10-08&quot;
    },
    {
      &quot;name&quot;: &quot;DUEAMOUNT&quot;,
      &quot;value&quot;: &quot;112550&quot;
    },
    {
      &quot;name&quot;: &quot;AMOUNT&quot;,
      &quot;value&quot;: &quot;1050&quot;
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
      <id>e3482ac3-2bee-4858-a48a-6d6dc6fa534f</id>
      <masked>false</masked>
      <name>link</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.tokenCus</defaultValue>
      <description></description>
      <id>c20ad823-c605-453c-bf29-e0273f9533de</id>
      <masked>false</masked>
      <name>tokenCus</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
