use std::sync::Arc;

use axum::{routing::post, Extension, Router};

use crate::{db::DbPool, handlers::user::{login, signup}};

pub fn get_routes(pool:Arc<DbPool>)->Router{
    Router::new().nest("/user", 
        Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
    ).layer(Extension(pool))
}