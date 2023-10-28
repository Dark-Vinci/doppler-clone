use sdk;
use tokio;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()>{
    let a = sdk::add(1, 3);
    println!("Hello, world!, {0}", a);
    Ok(())
}
