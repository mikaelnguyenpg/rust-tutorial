use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use sqlx::{Error, PgPool, Pool, Postgres, Transaction, postgres::PgPoolOptions};
use tokio::sync::Mutex;

pub type DbTransaction = Arc<Mutex<Transaction<'static, Postgres>>>;

pub struct Db {
    url: String,
    max_connection: u32,
}

impl Db {
    pub fn new() -> Self {
        Db {
            url: dotenv::var("DATABASE_URL").expect("DATABASE_URL must defined"),
            max_connection: 3,
        }
    }

    pub async fn connect(&self) -> Result<Pool<Postgres>, Error> {
        PgPoolOptions::new()
            .max_connections(self.max_connection)
            .connect(&self.url)
            .await
    }
}

pub async fn start_transaction(
    State(pool): State<PgPool>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Begin transaction
    let tx = pool
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    println!("Started transaction");

    let tx = Arc::new(Mutex::new(tx));

    // Inject into request extensions
    req.extensions_mut().insert(tx.clone());

    // Call the next handler
    let response = next.run(req).await;

    // Commit or rollback based on response status
    let tx = Arc::try_unwrap(tx)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_inner();

    if response.status().is_success() {
        tx.commit()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        println!("Commmited transaction");
    } else {
        tx.rollback()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        println!("Rollback transaction");
    }

    Ok(response)
}
