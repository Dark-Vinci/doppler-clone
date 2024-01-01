use async_trait::async_trait;

use crate::application::application::{App, Business};

#[async_trait]
impl Business for App {
    async fn get_business(&self) -> String {
        todo!()
    }

    async fn delete_business(&self) -> String {
        todo!()
    }
}
