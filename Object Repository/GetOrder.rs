<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GetOrder</name>
   <tag></tag>
   <elementGuidId>6cd8e90a-694b-4166-8010-c531fae9bc8a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://localhost:8899/mysql/customers/${customerID}/orders/${orderID}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'cust3760369'</defaultValue>
      <description></description>
      <id>0dcd9cc8-3fa6-4abd-8691-087ab8ad67ab</id>
      <masked>false</masked>
      <name>customerID</name>
   </variables>
   <variables>
      <defaultValue>'ord3588323'</defaultValue>
      <description></description>
      <id>9a85af4e-f99c-4e33-b69e-14d857a35432</id>
      <masked>false</masked>
      <name>orderID</name>
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
