<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>User registration</description>
   <name>Account-resource-User registration</name>
   <tag></tag>
   <elementGuidId>f2799201-6a52-4550-9d41-8167b10e8e11</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;activated\&quot;: true,\n  \&quot;authorities\&quot;: [\&quot;ROLE_USER\&quot;],\n  \&quot;companyName\&quot;: \&quot;CGA\&quot;,\n  \&quot;createdBy\&quot;: \&quot;system\&quot;,\n  \&quot;createdDate\&quot;: \&quot;2020-03-16T04:18:29.921Z\&quot;,\n  \&quot;email\&quot;: \&quot;jerry@localhost\&quot;,\n  \&quot;firstName\&quot;: \&quot;jerry\&quot;,\n  \&quot;id\&quot;: 0,\n  \&quot;imageUrl\&quot;: \&quot;\&quot;,\n  \&quot;langKey\&quot;: \&quot;en\&quot;,\n  \&quot;lastModifiedBy\&quot;: \&quot;system\&quot;,\n  \&quot;lastModifiedDate\&quot;: \&quot;2020-03-16T04:18:29.921Z\&quot;,\n  \&quot;lastName\&quot;: \&quot;Zhang\&quot;,\n  \&quot;login\&quot;: \&quot;jerry\&quot;,\n  \&quot;password\&quot;: \&quot;jerry\&quot;,\n  \&quot;userAccountDto\&quot;: {\n    \&quot;companyName\&quot;: \&quot;CGA\&quot;,\n    \&quot;userType\&quot;: \&quot;INDIVIDUAL\&quot;\n  },\n  \&quot;userType\&quot;: \&quot;INDIVIDUAL\&quot;\n}&quot;,
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
   <restUrl>http://localhost:9000/api/register</restUrl>
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
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
