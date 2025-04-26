import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import groovy.json.JsonSlurper



TestObject requestObjectGeocoding = findTestObject('Object Repository/Get_Geocoding')
def responseGeocoding = WS.sendRequest(requestObjectGeocoding)
WS.verifyResponseStatusCode(responseGeocoding, 200)
WS.verifyResponseStatusCodeInRange(responseGeocoding, 200, 299)


def jsonResponseGeocoding = new JsonSlurper().parseText(responseGeocoding.getResponseText())
String lat = jsonResponseGeocoding[0].lat.toString()
String lon = jsonResponseGeocoding[0].lon.toString()

GlobalVariable.latitude = lat
GlobalVariable.longitude = lon


TestObject requestObject = findTestObject('Object Repository/Get_5DaysForecast')

def response = WS.sendRequest(requestObject)
WS.verifyResponseStatusCode(response, 200)
WS.verifyResponseStatusCodeInRange(response, 200, 299)

def jsonResponse = new JsonSlurper().parseText(response.getResponseText())

String cityName = jsonResponse.city.name.toString()
println("nama kota: " + cityName)

double geoLat = Double.parseDouble(GlobalVariable.latitude.trim())
double geoLon = Double.parseDouble(GlobalVariable.longitude.trim())
double airpolutionLat = jsonResponse.city.coord.lat
double airpolutionLon = jsonResponse.city.coord.lon
def tolerance = 0.001
assert Math.abs(geoLat - airpolutionLat) <= tolerance?"Latitude match" : "Latitude doesn't match"
assert Math.abs(geoLon - airpolutionLon) <= tolerance?"Longitude match" : "Longitude doesn't match"