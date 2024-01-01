use crate::application::application::{App, Auth};
use async_trait::async_trait;

#[async_trait]
impl Auth for App {
    async fn sign_in_with_email(&self) -> String {
        todo!()
    }

    async fn sign_up_with_email(&self) -> String {
        todo!()
    }

    async fn sign_in_with_google(&self) -> String {
        todo!()
    }

    async fn sign_up_with_google(&self) -> String {
        todo!()
    }
}
