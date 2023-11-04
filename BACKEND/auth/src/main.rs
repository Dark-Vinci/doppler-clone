use anyhow::Result;
use dotenvy::dotenv;

use sdk;

use auth::application::app_data::AppData;

#[tokio::main]
async fn main() -> Result<()>{
    dotenv().ok();

    let PORT = env!("PORT");
    let ENVIRONMENT = env!("ENVIRONMENT");

    let a = AppData::new("a".to_owned(), "b".to_owned(), "s".to_owned());

    let a = sdk::add(1, 3);
    println!("{0}", a);
    println!("Hello, world!");

    Ok(())
}
