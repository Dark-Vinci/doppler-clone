use sdk;
use auth::app::app::App;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let app = App::new("a".to_owned(), "b".to_owned());

    let a = sdk::add(1, 3);
    println!("{0}", a);
    println!("Hello, world!");

    Ok(())
}
