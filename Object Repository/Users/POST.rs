<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST</name>
   <tag></tag>
   <elementGuidId>0bac26b8-c55f-4814-8d61-88fd736ed646</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;id\&quot;: 1,\n  \&quot;name\&quot;: \&quot;Mach Rojamz\&quot;,\n  \&quot;username\&quot;: \&quot;Ojam\&quot;,\n  \&quot;email\&quot;: \&quot;Ojam@jampirojam.id\&quot;,\n  \&quot;address\&quot;: {\n    \&quot;street\&quot;: \&quot;Bear Hunt Street\&quot;,\n    \&quot;suite\&quot;: \&quot;Apt. 456\&quot;,\n    \&quot;city\&quot;: \&quot;Ohio\&quot;,\n    \&quot;zipcode\&quot;: \&quot;451996-0405\&quot;,\n    \&quot;geo\&quot;: {\n      \&quot;lat\&quot;: \&quot;-32.3159\&quot;,\n      \&quot;lng\&quot;: \&quot;33.1496\&quot;\n    }\n  },\n  \&quot;phone\&quot;: \&quot;1-770-456-789 x54321\&quot;,\n  \&quot;website\&quot;: \&quot;ojam.id\&quot;,\n  \&quot;company\&quot;: {\n    \&quot;name\&quot;: \&quot;IT Blog\&quot;,\n    \&quot;catchPhrase\&quot;: \&quot;blog about tech\&quot;,\n    \&quot;bs\&quot;: \&quot;technology internet AI\&quot;\n  }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>cb34d3ba-7de9-47fb-acc3-99a104602e99</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://jsonplaceholder.typicode.com/users</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
