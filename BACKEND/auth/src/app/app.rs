use async_trait::async_trait;

#[derive(Debug)]
pub struct App {
    store: String,
    downstream: String,
}

#[async_trait]
pub trait Application {
    async fn login(&self) -> String;
    async fn sign_in(&self) -> String;
    async fn sign_in_with_google(&self) -> String;
}

impl App {
    pub fn new(store: String, downstream: String) -> Self {
        Self {
            store,
            downstream,
        }
    }
}