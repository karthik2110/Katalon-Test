<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Delete Collections</name>
   <tag></tag>
   <elementGuidId>14912f1e-2cd6-4d55-a24b-47442b0cb5fd</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;accountId\&quot;: \&quot;62de7fab813fa70c2943d218\&quot;,\n  \t\&quot;collectionIds\&quot;: [\n      \n      \&quot;63480ba7dc94e90a04e68454\&quot;, \n      \&quot;63480bd7dc94e90a04e68456\&quot;,\n      \&quot;63480caadc94e90a04e6845d\&quot;\n      \n    ]\n}&quot;,
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
      <webElementGuid>b84869a2-798a-476a-886e-3625ffa93856</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJraWQiOiJFZVBRWHZRYmx4OVwvMnliRzdxbkV2VDRva3VUbXlqcEpyeVU2bkxIRGFzMD0iLCJhbGciOiJSUzI1NiJ9.eyJzdWIiOiJjMjdjMzAxYi00YWEyLTRhMWQtOGU4YS02ZGEwMTMwNzFhOWMiLCJpc3MiOiJodHRwczpcL1wvY29nbml0by1pZHAudXMtZWFzdC0xLmFtYXpvbmF3cy5jb21cL3VzLWVhc3QtMV90YzNXQ3B5STMiLCJ2ZXJzaW9uIjoyLCJjbGllbnRfaWQiOiIxbGE0djk1ZThvNmFnMGdsMDNmZnRoaDRsbiIsIm9yaWdpbl9qdGkiOiJkZDZjN2Q1Yy0xYTNmLTRkMjMtYmExNS01Y2JkNDQyMDBkZTciLCJldmVudF9pZCI6ImFjOWE1OWUwLTgxNzEtNDRkZC1iYzg3LTBkOWY2ZTQ2NzAzNSIsInRva2VuX3VzZSI6ImFjY2VzcyIsInNjb3BlIjoib3BlbmlkIiwiYXV0aF90aW1lIjoxNjY1NjU4ODg3LCJleHAiOjE2NjU2NzAwNDEsImlhdCI6MTY2NTY2NjQ0MiwianRpIjoiZmNjZjZhMTAtN2FiYy00MWRhLThiMzMtMGIyYTAxNTg5OTk4IiwidXNlcm5hbWUiOiJjMjdjMzAxYi00YWEyLTRhMWQtOGU4YS02ZGEwMTMwNzFhOWMifQ.JdZcNns8tLqYdR8GEe-H5KGPCIXkojrKSGs0oK_wjormnqyZVfsA8kfKQWFeI6WEFLyd3Xppgx29Czrrm2pZ9imJ6Hf7Zp9UXxYISXN_EyCTRC5Jdccrm9r5PoBmqhAcRANRPQzYBmJgALBXPc05m3x5tsHG_szOjZQQbYe4ef1QqfLCxtNIlkrNy7xRaRTEMOE1jhaKMewY2lRVi0kAwP1w93V43IAXLpb5BPF7Oui6SjM1shFkrK1nRk_vCaFum4utVx1okAfkarYGBj2JXHjPORgrdMMI9_q_FThDqyyERD6zl-klPW_OYlr583R4dz4If_7vI0cfWJo7ptcF3g</value>
      <webElementGuid>245747cc-07b9-486d-bd13-128e3c3fb0c3</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.4.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>DELETE</restRequestMethod>
   <restUrl>https://admin-api-dev.paperflite.com/api/1.0/collections</restUrl>
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
