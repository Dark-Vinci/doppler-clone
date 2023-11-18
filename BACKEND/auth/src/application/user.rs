use crate::application::application::{App, User};

impl User for App {
    async fn get_user(&self) -> String {
        todo!()
    }

    async fn delete_user(&self) -> String {
        todo!()
    }

    async fn deactivated_user(&self) -> String {
        todo!()
    }

    async fn get_users(&self) -> Vec<String> {
        todo!()
    }
}
