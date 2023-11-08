use async_trait::async_trait;

#[async_trait]
pub trait Application {
    async fn sign_in_with_google(&self) -> String;
    async fn login(&self) -> String;
    async fn sign_in_with_email(&self) -> String;
    async fn sign_in_with_fb(&self) -> String;
}