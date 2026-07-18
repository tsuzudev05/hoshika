mod application;
mod domain;
mod infrastructure;
mod presentation;

use axum::Router;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tower_http::services::{ServeDir, ServeFile};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "hoshika=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let api_router = presentation::router::create_router(pool);

    // STATIC_DIR が設定されている場合のみ、ビルド済みフロントエンドを配信する
    // （Fly.io デプロイ時の単一アプリ構成用。ローカル/CI では未設定のため従来通り
    // API がルート直下で動く）。API は `/api` 配下にネストする。
    let app = match std::env::var("STATIC_DIR") {
        Ok(dir) => {
            let index_html = format!("{dir}/index.html");
            Router::new()
                .nest("/api", api_router)
                .fallback_service(ServeDir::new(&dir).not_found_service(ServeFile::new(index_html)))
        }
        Err(_) => api_router,
    };

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
