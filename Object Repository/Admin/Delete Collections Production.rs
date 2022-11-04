<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Delete Collections Production</name>
   <tag></tag>
   <elementGuidId>26b6d15e-85ad-49b6-90d4-b724b2627534</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;accountId\&quot;: \&quot;621f175af0a75221ca49328a\&quot;,\n  \t\&quot;collectionIds\&quot;: [\n      \n      \&quot;6349186b3183912c97e11c24\&quot;, \n      \&quot;6349183bf4c96c36b30d678e\&quot;\n      \n    ]\n}&quot;,
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
      <webElementGuid>40f9a383-a4f8-44c1-8ed8-494fd996e346</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJraWQiOiI3eW9MbWhNZCtibHZXRW1WT0xDRysrS2Z4WGhERkpEeDA1Q2ZUQUpjdlFJPSIsImFsZyI6IlJTMjU2In0.eyJzdWIiOiJkOGJmNTY1ZS1lYzBlLTQ5M2UtYjhkNi0xZGJlYzQwY2ZhNTQiLCJpc3MiOiJodHRwczpcL1wvY29nbml0by1pZHAudXMtZWFzdC0xLmFtYXpvbmF3cy5jb21cL3VzLWVhc3QtMV82SWlTNE9LUXQiLCJ2ZXJzaW9uIjoyLCJjbGllbnRfaWQiOiI5cnNlcGE4aDNxYm1xNDRjdG91dnZpaGJvIiwib3JpZ2luX2p0aSI6ImJmMjBkZWRlLTA3NWMtNGNkMC1iNzRhLWM4YzA0M2UzYjNiZCIsImV2ZW50X2lkIjoiNmJhMTY5MmItZmZiZS00Y2E4LTk5NmYtYjU1ZDBjNmRkMjRlIiwidG9rZW5fdXNlIjoiYWNjZXNzIiwic2NvcGUiOiJvcGVuaWQiLCJhdXRoX3RpbWUiOjE2NjU3MzQ4MjgsImV4cCI6MTY2NTczODQyOCwiaWF0IjoxNjY1NzM0ODI4LCJqdGkiOiIzM2MxN2I5YS03ZTVhLTQ4MWUtOGQ4Mi0zNTE0MzI4MzE4ZDEiLCJ1c2VybmFtZSI6ImQ4YmY1NjVlLWVjMGUtNDkzZS1iOGQ2LTFkYmVjNDBjZmE1NCJ9.FEovxmXBZ4L6RhIysqzHCj4R-AkwB-9Ghf3Nfqb2BUKqdwou7R9UuHInmue91Xri2ZGjyHZ1pI7SFU_puBGQLHgl-hwBD4s970CFvsKIRIDOiYSAGetLk6Dume6atWhaaHnAwi5qQx3d7aSZlJAgAI5AJ83JBr_bhvlW70zfLR0Hk1aciBPdhiNJtUFgqWLBKLSZxCFuDuXxOrf-mxlEip1hVIGnPb5ro4fRBmVmVvlHjhvE3bnddJR1PYIhGJeS9uO-0hso6CStAXR5AMbFnSXk6k5Vbh9kkbhMCOlMzHFw43AGAXahD-4A2MDsv1XQKortapxbeUk6iVhbOMPxlg</value>
      <webElementGuid>13718402-0ec8-4cfb-8e6b-665a7a81534b</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.4.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>DELETE</restRequestMethod>
   <restUrl>https://admin-api.paperflite.com/api/1.0/collections</restUrl>
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
