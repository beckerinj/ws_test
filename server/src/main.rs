use std::{net::SocketAddr, str::FromStr};

use axum::Router;

mod helpers;
mod ws;

const PORT: u32 = 4321;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let socket_addr = SocketAddr::from_str(&format!("0.0.0.0:{PORT}")).unwrap();

    println!("starting ws server on {socket_addr}");

    let app = Router::new().nest("/ws", ws::router());

    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
