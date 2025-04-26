<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET_CurrentAirPollution</name>
   <tag></tag>
   <elementGuidId>c69fbebe-ac3b-466a-9080-1bb89aa1dda8</elementGuidId>
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
      <id>01cec5a3-bbdc-4a09-86ff-2c55ca8227f0</id>
      <name>RIGHT_SCHEMA</name>
      <type>JSON_SCHEMA</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>JSONSchemas/air_pollution_schema.json</data>
      <activate>true</activate>
   </validationSteps>
   <variables>
      <defaultValue>GlobalVariable.API_KEY</defaultValue>
      <description></description>
      <id>b353be32-2019-4fbf-9138-5f47fad7fb10</id>
      <masked>false</masked>
      <name>API_KEY</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.BASE_URL</defaultValue>
      <description></description>
      <id>fb74dc27-7b45-4e03-b7ae-e67b37978771</id>
      <masked>false</masked>
      <name>BASE_URL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.AIR_POLLUTION_URL</defaultValue>
      <description></description>
      <id>b2f5f2ef-88ac-403a-abb0-e6fba9d3e35f</id>
      <masked>false</masked>
      <name>APP_URL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.latitude</defaultValue>
      <description></description>
      <id>091335d7-5d27-4d7b-a68a-42b6229187e3</id>
      <masked>false</masked>
      <name>latitude</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.longitude</defaultValue>
      <description></description>
      <id>b830aabe-bdcc-4bf2-9a89-2c238b6a1ba6</id>
      <masked>false</masked>
      <name>longitude</name>
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
