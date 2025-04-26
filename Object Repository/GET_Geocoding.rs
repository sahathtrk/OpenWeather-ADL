<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET_Geocoding</name>
   <tag></tag>
   <elementGuidId>3ec87021-6a9c-40e0-a5f4-36b4b238052c</elementGuidId>
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
   <restUrl>${BASE_URL}${APP_URL}?q=${VALID_CITY}&amp;limit=5&amp;appid=${API_KEY}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <validationSteps>
      <id>8fbf6b53-5160-4b95-86e1-34e7dd6f311b</id>
      <name>RIGHT_SCHEMA</name>
      <type>JSON_SCHEMA</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>JSONSchemas/geocoding_schema.json</data>
      <activate>true</activate>
   </validationSteps>
   <variables>
      <defaultValue>GlobalVariable.BASE_URL</defaultValue>
      <description></description>
      <id>f9f1a6bb-45bf-46fb-b601-42dda81fba16</id>
      <masked>false</masked>
      <name>BASE_URL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.GEOCODING_URL</defaultValue>
      <description></description>
      <id>b99f389c-99ec-4647-827a-ebea7aaf48ae</id>
      <masked>false</masked>
      <name>APP_URL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.API_KEY</defaultValue>
      <description></description>
      <id>8d17245e-bd2b-4a7a-8e4d-f4466c4076a7</id>
      <masked>false</masked>
      <name>API_KEY</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.VALID_CITY</defaultValue>
      <description></description>
      <id>4aa7bd7f-39e0-4c29-a96c-54e3c396522c</id>
      <masked>false</masked>
      <name>VALID_CITY</name>
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
