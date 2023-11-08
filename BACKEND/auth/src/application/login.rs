use crate::application::application::AppData;

pub struct Login<'a>(&'a AppData);

impl<'a> Login<'a> {
    pub fn new(data: &'a AppData) -> Self {
        Self(data)
    }

    pub fn with_email(&self) -> String {
        "email".to_owned()
    }

    pub fn with_google(&self) -> String {
        "google".to_owned()
    }

    pub fn with_facebook(&self) -> String {
        "facebook".to_owned()
    }
}
