use axum::{routing::get, Router};

use crate::handlers::health::health_check;

pub fn get_routes()->Router{
    Router::new().route("/health-check", get(health_check))
}