use crate::application::app_data::AppData;
use crate::application::app_trait::Application;
use crate::application::login::Login;
use crate::application::signup::SignUp;

pub struct Applications<'a> {
    data: &'a AppData,
    login: Login<'a>,
    sign_up: SignUp<'a>,
}

impl Applications {
    pub fn new(data: AppData) -> Self {
        Self {
            data: &data,
            login: Login::new(&data),
            sign_up: SignUp::new(&data),
        }
    }
}

impl Application for Applications {
    async fn sign_in_with_google(&self) -> String {
        self.
            login.
            withGoogle().
            await
    }

    async fn login(&self) -> String {
        self.
            login.
            withEmail().
            await
    }

    async fn sign_in_with_email(&self) -> String {
        self.
            login.
            with_mail().
            await
    }

    async fn sign_in_with_fb(&self) -> String {
        self.
            login.
            with_google().
            await
    }
}
