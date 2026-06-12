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