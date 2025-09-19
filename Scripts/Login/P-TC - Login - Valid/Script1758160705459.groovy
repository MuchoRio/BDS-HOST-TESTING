import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.configuration.RunConfiguration
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
import keywords.BaseConfiguration
import keywords.AppManager
import keywords.ScreenshotHelper

def config = BaseConfiguration.getBaseConfiguration()
String username = config.username
String password = config.password
int delay = config.delay
int timeout = config.timeout
String screenshotPath = 'Login/'


WebUI.openBrowser('')

AppManager.navigateURL()

WebUI.delay(delay)

ScreenshotHelper.takeScreenshotPath(screenshotPath, "01.png")

WebUI.setText(findTestObject('Object Repository/Login/Page_Bank Mega Branch/01. TextField-Username'), username, FailureHandling.STOP_ON_FAILURE )

WebUI.delay(delay)

ScreenshotHelper.takeScreenshotPath(screenshotPath, "02.png")

WebUI.setText(findTestObject('Login/Page_Bank Mega Branch/01. TextField-Password'), password, FailureHandling.STOP_ON_FAILURE)

WebUI.delay(delay)

ScreenshotHelper.takeScreenshotPath(screenshotPath, '03.png')

WebUI.click(findTestObject('Login/Page_Bank Mega Branch/01. Button-Login'))

WebUI.delay(delay)

boolean objectDashboard = WebUI.waitForElementPresent(findTestObject('Login/Page_Bank Mega Branch/02. Page-Dashboard'), timeout)
boolean objectDashboard_title = WebUI.waitForElementPresent(findTestObject('Object Repository/Login/Page_Bank Mega Branch/02. Text-Dashboard-Nomor Referensi Transaksi'), timeout)

if (objectDashboard || objectDashboard_title) {
	ScreenshotHelper.takeScreenshotPath(screenshotPath, '04.png')
}

//WebUI.closeBrowser()

