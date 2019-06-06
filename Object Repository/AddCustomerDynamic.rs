<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>AddCustomerDynamic</name>
   <tag></tag>
   <elementGuidId>cd84416e-09fe-45b2-8c92-751e0f986d75</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;firstName\&quot;:\&quot;${firstName}\&quot;,\n    \&quot;lastName\&quot;:\&quot;${lastName}\&quot;,\n    \&quot;phone\&quot;: ${phone},\n    \&quot;email\&quot;: \&quot;${email}\&quot;,\n    \&quot;address\&quot;:\&quot;${address}\&quot;,\n    \&quot;city\&quot;: \&quot;${city}\&quot;,\n    \&quot;state\&quot;: \&quot;${state}\&quot;,\n    \&quot;zipcode\&quot;:${zipcode},\n    \&quot;country\&quot;: \&quot;${country}\&quot;\n  }&quot;,
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
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://localhost:8899/mysql/customers/</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'mm300'</defaultValue>
      <description></description>
      <id>c47287c6-3cc7-4ad6-bdf6-77b14562bb30</id>
      <masked>false</masked>
      <name>firstName</name>
   </variables>
   <variables>
      <defaultValue>'mm300'</defaultValue>
      <description></description>
      <id>ee3daaa0-eb0f-47a8-bece-145f58c72810</id>
      <masked>false</masked>
      <name>lastName</name>
   </variables>
   <variables>
      <defaultValue>3800</defaultValue>
      <description></description>
      <id>32aa17ae-f46a-45b6-bb49-61fb1d5511db</id>
      <masked>false</masked>
      <name>phone</name>
   </variables>
   <variables>
      <defaultValue>'m380@kps.com'</defaultValue>
      <description></description>
      <id>6ee3a2a0-0414-446c-b61f-909f321263b8</id>
      <masked>false</masked>
      <name>email</name>
   </variables>
   <variables>
      <defaultValue>'Parkstrasse3\r\n'</defaultValue>
      <description></description>
      <id>dbc57468-0ad9-4f24-833a-b17691c0d948</id>
      <masked>false</masked>
      <name>address</name>
   </variables>
   <variables>
      <defaultValue>'Bonn'</defaultValue>
      <description></description>
      <id>b96902c9-e465-4ae0-a5f5-685df004cafb</id>
      <masked>false</masked>
      <name>city</name>
   </variables>
   <variables>
      <defaultValue>'NRW'</defaultValue>
      <description></description>
      <id>ef2b7def-3463-4da6-a3af-ed4b103c85ff</id>
      <masked>false</masked>
      <name>state</name>
   </variables>
   <variables>
      <defaultValue>44369</defaultValue>
      <description></description>
      <id>b453a9fa-684a-4d60-9a52-5409c640b020</id>
      <masked>false</masked>
      <name>zipcode</name>
   </variables>
   <variables>
      <defaultValue>'Germany'</defaultValue>
      <description></description>
      <id>e8e9cc73-b85b-4ed1-a23b-dcdc694bb6f3</id>
      <masked>false</masked>
      <name>country</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
