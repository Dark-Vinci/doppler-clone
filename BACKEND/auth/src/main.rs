use anyhow::Result;
use dotenvy::dotenv;

// use auth::application::app_data::AppData;
use auth::application::application::App;
use auth::config::config::Config;
use sdk;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    // config

    // repository

    // service

    // controller

    let abc = sdk::add(2, 3);

    let config = Config::new();

    let a = App::new("".to_string(), "".to_string(), config);


    let a = sdk::add(1, 3);
    println!("{0}", a);
    println!("Hello, world!");

    Ok(())
}
