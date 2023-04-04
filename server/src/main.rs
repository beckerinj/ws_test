use std::{net::SocketAddr, str::FromStr, sync::Arc};

use axum::{Extension, Router};
use config::Env;

mod config;
mod helpers;
mod ws;

const PORT: u32 = 4321;

type EnvExtension = Extension<Arc<Env>>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env = config::load();

    let socket_addr = SocketAddr::from_str(&format!("0.0.0.0:{PORT}")).unwrap();

    println!(
        "starting ws server on {socket_addr} | random disconnect: {}",
        env.random_disconnect
    );

    let app = Router::new()
        .nest("/ws", ws::router())
        .layer(Extension(Arc::new(env)));

    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
