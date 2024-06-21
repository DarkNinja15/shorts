use std::sync::Arc;

use axum::{middleware, routing::{get, post}, Extension, Router};

use crate::{db::DbPool, handlers::shorts::{create_short, get_shorts}, middleware::auth_middleware::auth_middleware};

pub fn get_routes(pool:Arc<DbPool>)->Router{
    Router::new().nest("/shorts", Router::new()
        .route("/", get(get_shorts))
        .route("/create-short", post(create_short))
    ).layer(Extension(pool)).route_layer(middleware::from_fn(auth_middleware))
}