<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CreateProduct</name>
   <tag></tag>
   <elementGuidId>501a78f6-6da8-46e4-9694-7aff3ccbfc58</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n \&quot;productName\&quot; :\&quot;${productName}\&quot;,\n \&quot;cost\&quot; : ${cost},\n \&quot;trackingID\&quot; :\&quot;${trackingID}\&quot;\n}&quot;,
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
   <restUrl>http://localhost:8899/mysql/customers/${customerID}/orders/${myorderID}/products</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'cust3244213'</defaultValue>
      <description></description>
      <id>245228b8-144d-4e6a-8040-e66fcad7f130</id>
      <masked>false</masked>
      <name>customerID</name>
   </variables>
   <variables>
      <defaultValue>'ord4370227'</defaultValue>
      <description></description>
      <id>9b7538ac-1284-4897-9f2f-7ba705f8688c</id>
      <masked>false</masked>
      <name>myorderID</name>
   </variables>
   <variables>
      <defaultValue>'G30DrivingBelt'</defaultValue>
      <description></description>
      <id>dd382efd-103e-4ca6-9fdb-2946c3945af7</id>
      <masked>false</masked>
      <name>productName</name>
   </variables>
   <variables>
      <defaultValue>250</defaultValue>
      <description></description>
      <id>f4e2dd51-c90e-4a66-873c-1080e36ff5e7</id>
      <masked>false</masked>
      <name>cost</name>
   </variables>
   <variables>
      <defaultValue>'ID-0-16'</defaultValue>
      <description></description>
      <id>3c3193a6-a7d9-44a0-97d6-af6badcf45cf</id>
      <masked>false</masked>
      <name>trackingID</name>
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
