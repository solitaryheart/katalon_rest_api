<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>AddCustomerID</name>
   <tag></tag>
   <elementGuidId>336a7093-3a2b-4dae-85bc-74a4b2aa124c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;firstName\&quot;:\&quot;heini28\&quot;,\n    \&quot;lastName\&quot;:\&quot;meier\&quot;,\n    \&quot;phone\&quot;: 7574,\n    \&quot;email\&quot;: \&quot;mm41@kps.com\&quot;,\n    \&quot;address\&quot;:\&quot;Badway\&quot;,\n    \&quot;city\&quot;: \&quot;Dortmund\&quot;,\n    \&quot;state\&quot;: \&quot;NRW\&quot;,\n    \&quot;zipcode\&quot;:68400,\n    \&quot;country\&quot;: \&quot;England\&quot;\n  }&quot;,
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
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()






WS.verifyElementPropertyValue(response, 'issues[0].fields.project.key', 'KTP')


WS.verifyElementPropertyValue(response, 'issues[0].fields.project.key', 'KTP')


WS.verifyElementPropertyValue(response, 'issues[0].fields.project.key', 'KTP')


WS.verifyElementPropertyValue(response, 'issues[0].fields.project.key', 'KTP')


WS.verifyElementPropertyValue(response, 'issues[0].fields.project.key', 'KTP')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
