use anyhow::Result;
use dotenvy::dotenv;

use auth::application::app_data::AppData;
use auth::application::application::Applications;
use sdk;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let abc = sdk::add(2, 3);

    let PORT = env!("PORT");
    let ENVIRONMENT = env!("ENVIRONMENT");
    let DB_PASSWORD = env!("DB_PASSWORD");
    let DB_HOST = env!("DB_HOST");
    let DB_NAME = env!("DB_NAME");
    let DB_USERNAME = env!("DB_USERNAME");

    let a = AppData::new("a".to_owned(), "b".to_owned(), "s".to_owned());

    let app = Applications::new(a);

    // let res = app.sign_in_with_email().await;

    let a = sdk::add(1, 3);
    println!("{0}", a);
    println!("Hello, world!");

    Ok(())
}
