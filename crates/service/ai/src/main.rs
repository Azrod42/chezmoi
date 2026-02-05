use lambda_http::{run, service_fn, Error};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::sync::Arc;
use tracing::info;

mod handlers;

#[derive(Clone)]
pub(crate) struct State {
    pool: PgPool,
}

impl State {
    pub(crate) fn pool(&self) -> &PgPool {
        &self.pool
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_target(false)
        .without_time()
        .init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL missing");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("failed to connect to postgres");

    info!("ai started");

    let state = Arc::new(State { pool });

    run(service_fn(move |req| {
        let state = state.clone();
        async move { handlers::router(req, state).await }
    }))
    .await
}
