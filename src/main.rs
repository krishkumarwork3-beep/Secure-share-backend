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