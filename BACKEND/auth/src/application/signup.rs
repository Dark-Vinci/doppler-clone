use crate::application::application::AppData;

pub struct SignUp<'a>(&'a AppData);

impl<'a> SignUp<'a> {
    pub fn new(data: &'a AppData) -> Self {
        Self(data)
    }

    pub async fn with_mail(&self) -> String {
        "email".to_owned()
    }

    pub async fn with_google(&self) -> String {
        "google".to_owned()
    }

    pub async fn github(&self) -> String {
        "github".to_owned()
    }
}
