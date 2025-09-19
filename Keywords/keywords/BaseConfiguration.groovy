package keywords

import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.configuration.RunConfiguration
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testcase.TestCase
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows

import internal.GlobalVariable

public class BaseConfiguration {

	@Keyword
	static def getBaseConfiguration() {
		String 	WEB_URL 			= GlobalVariable.G_BaseURL?.toString()?.trim()
		String	USERNAME_STAFF 		= GlobalVariable.G_Username_Staff?.toString()?.trim()
		String 	PASSWORD_STAFF 		= GlobalVariable.G_Password_Staff?.toString()?.trim()
		String	USERNAME_SPV		= GlobalVariable.G_Username_SPV?.toString()?.trim()
		String	PASSWORD_SPV		= GlobalVariable.G_Password_SPV?.toString()?.trim()
		Integer DELAY_DURATION 		= (GlobalVariable.G_Delay_Duration != null ? (GlobalVariable.G_Delay_Duration as Number).intValue() : 2)
		Integer TIMEOUT_DURATION 	= (GlobalVariable.G_Timeout_Duration != null ? (GlobalVariable.G_Timeout_Duration as Number).intValue() : 30)
		String 	PROJECTPATH 		= RunConfiguration.getProjectDir()
		String 	SCREENSHOTPATH 		= GlobalVariable.G_Screenshot_Path

		assert WEB_URL			: "WEB_URL belum diisi!"
		assert USERNAME_STAFF	: "USERNAME_STAFF belum diisi!"
		assert PASSWORD_STAFF	: "PASSWORD_STAFF belum diisi!"
		assert USERNAME_SPV		: "USERNAME_SPV belum diisi!"
		assert PASSWORD_SPV		: "PASSWORD_SPV belum diisi!"

		return [
			web_url						: WEB_URL,
			username_staff				: USERNAME_STAFF,
			password_staff				: PASSWORD_STAFF,
			username_spv				: USERNAME_SPV,
			password_spv				: PASSWORD_SPV,
			delay_duration				: DELAY_DURATION,
			timeout_duration			: TIMEOUT_DURATION,
			projectpath					: PROJECTPATH,
			screenshotpath				: SCREENSHOTPATH

		]
	}
}
