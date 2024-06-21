
use std::sync::Arc;

use axum::{extract::Path, http::{HeaderMap, StatusCode}, response::IntoResponse, Extension, Json};
use serde_json::{json, Value};

use crate::db::DbPool;

use super::user::Claims;

pub async fn get_shorts(Extension(claim):Extension<Claims>){}

pub async fn create_short(Extension(claim):Extension<Claims>, Json(req): Json<Value>,Path(author):Path<String>,Extension(pool):Extension<Arc<DbPool>>,head:HeaderMap)->Result<impl IntoResponse, StatusCode>{
    // print the headers
    tracing::debug!("claims: {:?}", claim.sub);
    
    let mut conn=pool.get().expect("Failed to get database connection from pool");

    let ref_url=match req.get("ref_url") {
        Some(url)=>url.as_str().ok_or(StatusCode::BAD_REQUEST)?,
        None=>return Ok((StatusCode::BAD_REQUEST,Json(json!({"error":"ref_url is required"}))))
    };

    let title=match req.get("title") {
        Some(t)=>t.as_str().ok_or(StatusCode::BAD_REQUEST)?,
        None=>return Ok((StatusCode::BAD_REQUEST,Json(json!({"error":"title is required"}))))
    };

    let description=match req.get("description") {
        Some(d)=>d.as_str().ok_or(StatusCode::BAD_REQUEST)?,
        None=>return Ok((StatusCode::BAD_REQUEST,Json(json!({"error":"description is required"}))))
    };


    Ok((StatusCode::OK,Json(json!({"message":"Short created successfully"}))))
}