use std::net::SocketAddr;

use anyhow::Result;
use axum::Server;

use gateway::controllers::controller::Controller;
use gateway::helper::helper::graceful_shutdown;
use sdk;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Controller::new();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    let instance = Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown((graceful_shutdown()))
        .await;

    if let Err(e) = instance {
        println!("STARTUP ERROR😭: {e} happened")
    }

    println!("SUCCESS 🤭: Application listening/running on address: {addr} ✈️🚀");
    Ok(())
}

