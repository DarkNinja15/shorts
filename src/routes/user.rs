use axum::{routing::post, Router};

use crate::handlers::user::{create_user, login};

pub fn get_routes()->Router{
    Router::new().nest("/user", 
        Router::new()
        .route("/signup", post(create_user))
        .route("/login", post(login))
    )
}