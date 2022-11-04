
/**
 * This class is generated automatically by Katalon Studio and should not be modified or deleted.
 */

import java.lang.String



def static "Login.NavigatestoStudentLoginPage"() {
    (new Login()).NavigatestoStudentLoginPage()
}


def static "Login.EnteringValidStudentCredentials"(
    	String Username	
     , 	String Password	) {
    (new Login()).EnteringValidStudentCredentials(
        	Username
         , 	Password)
}


def static "Login.SubmitButtonClick"() {
    (new Login()).SubmitButtonClick()
}


def static "Login.VerifyStudentHomePage"() {
    (new Login()).VerifyStudentHomePage()
}
