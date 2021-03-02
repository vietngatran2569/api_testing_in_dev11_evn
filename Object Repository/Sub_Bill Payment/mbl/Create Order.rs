<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Order</name>
   <tag></tag>
   <elementGuidId>95715ea3-edcc-464d-acc7-c657fcec74d2</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;payment_method\&quot;: \&quot;ayapay_ph_no\&quot;,\n  \&quot;payment_method_title\&quot;: \&quot;AYA PAY\&quot;,\n  \&quot;set_paid\&quot;: true,\n  \&quot;billing\&quot;: {\n    \&quot;first_name\&quot;: \&quot;Jan\&quot;,\n    \&quot;address_1\&quot;: \&quot;27 Park Road\&quot;,\n    \&quot;city\&quot;: \&quot;Yangon\&quot;,\n    \&quot;email\&quot;: \&quot;jan@example.com\&quot;,\n    \&quot;phone\&quot;: \&quot;0912345678\&quot;\n  },\n   \&quot;status\&quot;: \&quot;completed\&quot;,\n  \&quot;line_items\&quot;: [\n    {\n      \&quot;product_id\&quot;: 21,\n      \&quot;quantity\&quot;: 1\n    }\n  ]\n}&quot;,
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
   <restUrl>https://mbl.ayapay.com/wp-json/wc/v3/orders</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
