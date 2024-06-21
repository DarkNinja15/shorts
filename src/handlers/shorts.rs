
use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use diesel::RunQueryDsl;
use serde_json::{json, Value};
use crate::schema::shorts::dsl::*;

use crate::db::DbPool;
use crate::utils::{get_uid, Claims};


pub async fn get_shorts(){}

pub async fn create_short(Extension(claim):Extension<Claims>, Json(req): Json<Value>,Extension(pool):Extension<Arc<DbPool>>)->Result<impl IntoResponse, StatusCode>{
    let mut conn=pool.get().expect("Failed to get database connection from pool");

    let _ref_url=match req.get("ref_url") {
        Some(url)=>url.as_str().ok_or(StatusCode::BAD_REQUEST)?,
        None=>return Ok((StatusCode::BAD_REQUEST,Json(json!({"error":"ref_url is required"}))))
    };

    let _title=match req.get("title") {
        Some(t)=>t.as_str().ok_or(StatusCode::BAD_REQUEST)?,
        None=>return Ok((StatusCode::BAD_REQUEST,Json(json!({"error":"title is required"}))))
    };

    let _description=match req.get("description") {
        Some(d)=>d.as_str().ok_or(StatusCode::BAD_REQUEST)?,
        None=>return Ok((StatusCode::BAD_REQUEST,Json(json!({"error":"description is required"}))))
    };

    let new_short=crate::models::shorts::Shorts{
        id:get_uid().clone(),
        ref_url:_ref_url.to_string(),
        title:_title.to_string(),
        description:_description.to_string(),
        author:claim.sub.clone()
    };

    

    let result=diesel::insert_into(shorts).values(&new_short).execute(&mut conn).map_err(|_| {(StatusCode::INTERNAL_SERVER_ERROR,Json(json!({"error":"Unable to insert short in db."})))});

    tracing::debug!("result: {:?}", result);

    if let Err((code,json))=result{
        return Ok((code,json));
    }
    
    Ok((StatusCode::OK,Json(json!({"message":"Short created successfully","short":new_short}))))
}