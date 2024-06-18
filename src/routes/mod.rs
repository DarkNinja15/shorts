use std::sync::Arc;

use axum::Router;

use crate::db::DbPool;

pub mod health;
pub mod shorts;
pub mod user;

pub fn create_routes(pool: Arc<DbPool>)->Router{
    Router::new().merge(health::get_routes()).merge(user::get_routes(pool)).merge(shorts::get_routes())
}