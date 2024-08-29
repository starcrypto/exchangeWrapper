mod config;
use crate::config::Config;
use anyhow::Result;
use clap::Parser;

use async_session::MemoryStore;
use axum::Router;
use std::net::TcpListener;
use tower_http::services::ServeDir;

async fn get_router() -> Router {
    let mut auth_db = AuthDB::new();
    auth_db
        .register_user("bob", Secret::from("secret".to_string()), "Robert")
        .await;
    auth_db
        .register_public_client(
            "LocalClient",
            "https://www.thunderclient.com/oauth/callback",
            "account:read",
        )
        .await;
    let state = oauth::state::State::new(auth_db.clone());
    let sessions = MemoryStore::new();
    let state = AppState {
        sessions,
        state,
        databbase: auth_db,
    };
    Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .nest("/oauth", crate::oauth::routes::routes())
        .nest("/api", routes::routes())
        .with_state(state)
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut config = Config::parse();
    config.load()?;
    let port = config.inner.port;

    let router = get_router().await;
    let addr = if let Some(baddr) = config.inner.bind_addr {
        format!("{baddr}:{port}")
    } else {
        format!("0.0.0.0:{port}")
    };
    let listener = TcpListener.bind(addr)?;
    axum::Server::from_tcp(listener)?
        .serve(router.into_make_service())
        .await
}
