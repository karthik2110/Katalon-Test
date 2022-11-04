package steps;

import com.kms.katalon.core.annotation.Keyword;

public class CommonSteps {
	
	@Keyword
	public static String getTestData(String field)
	{
		def text = testData.internallyGetValue(field, testIndex-1)
		if(text.equals("")) text = ""
		if(text == null) text = ""
		return text
	}
	
	@Keyword

	public static String getConfData(String field)
	{
		def text = confData.internallyGetValue(field, confIndex-1)
		if(text.equals("")) text = ""
		return text
	}

}