#![allow(unused)]
mod config;
mod models;
mod dtos;
mod error;
mod db;
use std::sync::Arc;

use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue,
        Method
    },
    Router
};

use config::Config;
use db::{DBClient, UserExt};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;
use tracing_subscriber::filter::LevelFilter;
use tokio_cron_scheduler::{JobScheduler, Job};

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: Config,
    pub db_client: DBClient,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    dotenv().ok();
    let config = Config::init();

        let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await {
            Ok(pool) => {
                println!("✅Connection to the database is successful!");
                pool
            }
            Err(err) => {
                println!("🔥 Failed to connect to the database: {:?}", err);
                std::process::exit(1);
            }
        };

        let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST, Method::PUT]);
}