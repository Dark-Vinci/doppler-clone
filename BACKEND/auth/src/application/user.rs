use crate::application::application::{App, User};

impl User for App {
    async fn get_one(&self) -> String {
        todo!()
    }

    async fn delete(&self) -> String {
        todo!()
    }

    async fn deactivated(&self) -> String {
        todo!()
    }

    async fn get_many(&self) -> Vec<String> {
        todo!()
    }
}
