use sdk;
use tokio::main;
use anyhow::Result;

#[main]
async fn main() -> Result<()>{
    // let b = gateway::
    let a = sdk::add(1, 3);
    println!("Hello, world!, {0}", a);
    Ok(())
}
