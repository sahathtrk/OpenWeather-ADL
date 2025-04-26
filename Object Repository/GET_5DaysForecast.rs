<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET_5DaysForecast</name>
   <tag></tag>
   <elementGuidId>827f04f5-4464-4ee0-9c89-ef180ca549a5</elementGuidId>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>9.3.2</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${BASE_URL}${APP_URL}?lat=${latitude}&amp;lon=${longitude}&amp;appid=${API_KEY}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <validationSteps>
      <id>56416322-489c-41a6-863c-c55e35e5918b</id>
      <name>RIGHT_SCHEMA</name>
      <type>JSON_SCHEMA</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>JSONSchemas/forecast_schema.json</data>
      <activate>true</activate>
   </validationSteps>
   <variables>
      <defaultValue>GlobalVariable.API_KEY</defaultValue>
      <description></description>
      <id>811f8321-7759-4034-ab97-742688d13ea2</id>
      <masked>false</masked>
      <name>API_KEY</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.latitude</defaultValue>
      <description></description>
      <id>b1605a4e-6680-4418-b87e-12928f92588b</id>
      <masked>false</masked>
      <name>latitude</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.longitude</defaultValue>
      <description></description>
      <id>f92fcbd2-c1dd-423b-a026-32c9c2759c0d</id>
      <masked>false</masked>
      <name>longitude</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.BASE_URL</defaultValue>
      <description></description>
      <id>8f7851df-cc9f-4ac8-b4c4-76a3593577fe</id>
      <masked>false</masked>
      <name>BASE_URL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.FORECAST_URL</defaultValue>
      <description></description>
      <id>f1639b21-ddac-4525-85a9-91a5155720f1</id>
      <masked>false</masked>
      <name>APP_URL</name>
   </variables>
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
