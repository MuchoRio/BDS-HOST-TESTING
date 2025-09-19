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
import com.kms.katalon.core.testobject.ConditionType as ConditionType
import keywords.BaseConfiguration as BaseConfiguration
import keywords.ScreenshotHelper as ScreenshotHelper
import org.openqa.selenium.WebDriver as WebDriver
import org.openqa.selenium.WebElement as WebElement
import org.openqa.selenium.By as By
import com.kms.katalon.core.webui.driver.DriverFactory as DriverFactory
import java.util.Random as Random

def config = BaseConfiguration.getBaseConfiguration()

int delay = config.delay

int timeout = config.timeout

String screenshotPath = 'Dashboard/SetorTunai'

WebUI.callTestCase(findTestCase('Test Cases/Login/P-TC - Login - Valid'), [:], FailureHandling.STOP_ON_FAILURE)

//start setor tunai test
WebUI.click(findTestObject('Object Repository/Dashboard/Setor Tunai/Page_Bank Mega Branch/02. Menu-Setor Tunai'))

WebUI.delay(delay)

ScreenshotHelper.takeScreenshotPath(screenshotPath, '01.png')

WebUI.click(findTestObject('Object Repository/Dashboard/Setor Tunai/Page_Bank Mega Branch/02. Option-Type Rate'))

WebUI.delay(delay)

ScreenshotHelper.takeScreenshotPath(screenshotPath, '02.png')

WebUI.setText(findTestObject('Object Repository/Dashboard/Setor Tunai/Page_Bank Mega Branch/02. Input-No ID'), generateRandomNumber())

WebUI.delay(delay)

ScreenshotHelper.takeScreenshotPath(screenshotPath, '03.png')

// ==== Dropdown Jenis ID (Random) ====
WebUI.click(findTestObject('Object Repository/Dashboard/Setor Tunai/Page_Bank Mega Branch/02. Dropdown-Jenis ID'))

WebDriver driver = DriverFactory.getWebDriver()

List<WebElement> options = driver.findElements(By.xpath('//div[contains(@class,\'column-1\')]'))

int totalOptions = options.size()

WebUI.comment('Total opsi ditemukan: ' + totalOptions)

if (totalOptions > 0) {
    Random rand = new Random()

    int randomIndex = rand.nextInt(totalOptions)

    WebUI.comment((('Index yang dipilih: ' + randomIndex) + ' -> ') + options.get(randomIndex).getText())

    options.get(randomIndex).click()
} else {
    println('Tidak ada opsi dropdown ditemukan!')
}

// =======

WebUI.delay(delay)

ScreenshotHelper.takeScreenshotPath(screenshotPath, '04.png')

WebUI.setText(findTestObject('Object Repository/Dashboard/Setor Tunai/Page_Bank Mega Branch/02. Input-Deal Number'), generateRandomNumber())

WebUI.delay(delay)

ScreenshotHelper.takeScreenshotPath(screenshotPath, '05.png')

WebUI.setText(findTestObject('Object Repository/Dashboard/Setor Tunai/Page_Bank Mega Branch/02. Input-Nomor Rekening'), 
    generateRandomNumber())

WebUI.delay(delay)

ScreenshotHelper.takeScreenshotPath(screenshotPath, '06.png')

WebUI.setText(findTestObject('Object Repository/Dashboard/Setor Tunai/Page_Bank Mega Branch/02. Input - Nominal Yang Disetor'), 
    GlobalVariable.G_Nominal_Disetor)

WebUI.delay(delay)

ScreenshotHelper.takeScreenshotPath(screenshotPath, '07.png')

WebUI.setText(findTestObject('Object Repository/Dashboard/Setor Tunai/Page_Bank Mega Branch/02. Input - Nominal Yang Dikredit'), 
    GlobalVariable.G_Nominal_Dikredit)

WebUI.delay(delay)

ScreenshotHelper.takeScreenshotPath(screenshotPath, '08.png')

WebUI.setText(findTestObject('Dashboard/Setor Tunai/Page_Bank Mega Branch/02. Textarea-Remarks'), 
    GlobalVariable.G_Remarks_Message)

WebUI.delay(delay)

ScreenshotHelper.takeScreenshotPath(screenshotPath, '09.png')

WebUI.click(findTestObject('Object Repository/Dashboard/Setor Tunai/Page_Bank Mega Branch/02. Button-Submit'))

WebUI.waitForElementPresent(findTestObject('Dashboard/Setor Tunai/Page_Bank Mega Branch/02. Popup-Status Transaksi'), timeout)

WebUI.delay(delay)

ScreenshotHelper.takeScreenshotPath(screenshotPath, '10.png')

WebUI.closeBrowser()

String generateRandomNumber(int length = 10) {
    Random rand = new Random()

    StringBuilder sb = new StringBuilder()

    for (int i = 0; i < length; i++) {
        sb.append(rand.nextInt(10))
    }
    
    return sb.toString()
}

