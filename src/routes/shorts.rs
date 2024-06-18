use axum::{routing::get, Router};

use crate::handlers::shorts::get_shorts;

pub fn get_routes()->Router{
    Router::new().route("/shorts", get(get_shorts))
}