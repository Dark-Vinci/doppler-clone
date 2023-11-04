use std::net::SocketAddr;

use anyhow::Result;
use axum::ServiceExt;
use tokio::signal;

use gateway::controllers::controller::Controller;
use sdk;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Controller::new();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown((graceful_shutdown_handler()))
        .await
        .unwrap();

    println!("application listening on addr: {addr} ", addr = addr);
    Ok(())
}

async fn graceful_shutdown_handler() -> () {
    let crt_l = async {
        signal::ctrl_c()
            .await
            .expect("failed to install CRTL-C handler")
    };

    #[cfg(unix)]
        let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await
    };

    #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = crt_l => {},
        _ = terminate => {},
    }

    println!("Signal received: handling graceful shutdown server")
}