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

WebUI.openBrowser('')

WebUI.navigateToUrl('http://10.14.20.156:3000/bankmega')

WebUI.setText(findTestObject('Object Repository/Debug/Page_Bank Mega Branch/input_Sign In_sign-user'), 'UATSTAFF2')

WebUI.setEncryptedText(findTestObject('Object Repository/Debug/Page_Bank Mega Branch/input_Password_sign-pass'), 'Ww1FHTu5Bdxje2DIuZaruw==')

WebUI.click(findTestObject('Object Repository/Debug/Page_Bank Mega Branch/button_Continue'))

WebUI.click(findTestObject('Object Repository/Debug/Page_Bank Mega Branch/td_Setor Tunai'))

WebUI.setText(findTestObject('Object Repository/Debug/Page_Bank Mega Branch/input_No. ID  Jenis ID  Jenis Pelaku_form-c_6c50d4'), 
    '1234')

WebUI.setText(findTestObject('Object Repository/Debug/Page_Bank Mega Branch/input_No. ID  Jenis ID  Jenis Pelaku_form-c_746734'), 
    '')

WebUI.click(findTestObject('Object Repository/Debug/Page_Bank Mega Branch/input_No. ID  Jenis ID  Jenis Pelaku_form-c_746734'))

WebUI.click(findTestObject('Object Repository/Debug/Page_Bank Mega Branch/div_Type Rate         Rate EMX     Adjust Rate'))

WebUI.click(findTestObject('Object Repository/Debug/Page_Bank Mega Branch/input_Type Rate_rtespc'))

WebUI.setText(findTestObject('Object Repository/Debug/Page_Bank Mega Branch/input_No. ID  Jenis ID  Jenis Pelaku_form-c_6c50d4'), 
    '1234')

WebUI.setText(findTestObject('Object Repository/Debug/Page_Bank Mega Branch/input_No. ID  Jenis ID  Jenis Pelaku_form-c_746734'), 
    '')

WebUI.click(findTestObject('Object Repository/Debug/Page_Bank Mega Branch/input_No. ID  Jenis ID  Jenis Pelaku_form-c_746734'))

WebUI.click(findTestObject('Object Repository/Debug/Page_Bank Mega Branch/div_Adjust Rate'))

WebUI.setText(findTestObject('Object Repository/Debug/Page_Bank Mega Branch/input_Deal Number_form-control form-control-sm'), 
    '12345')

WebUI.setText(findTestObject('Object Repository/Debug/Page_Bank Mega Branch/input_Nomor Rekening Escrow  Jenis Rekening_c7a997'), 
    '1232432')

WebUI.setText(findTestObject('Object Repository/Debug/Page_Bank Mega Branch/input_Nominal yang disetor_form-control for_e488a2'), 
    '200,000')

WebUI.setText(findTestObject('Object Repository/Debug/Page_Bank Mega Branch/input_Nominal yang dikredit ke rekening_for_d45350'), 
    '2,000')

WebUI.setText(findTestObject('Object Repository/Debug/Page_Bank Mega Branch/textarea_Remarks_form-control form-control-sm'), 
    'debug')

WebUI.click(findTestObject('Object Repository/Debug/Page_Bank Mega Branch/button_Submit                    Transaksi'))

WebUI.click(findTestObject('Object Repository/Debug/Page_Bank Mega Branch/button_Submit                    Transaksi'))

WebUI.click(findTestObject('Object Repository/Debug/Page_Bank Mega Branch/h2_Transaksi Gagal'))

