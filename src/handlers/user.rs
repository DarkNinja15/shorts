use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use bcrypt::{verify, DEFAULT_COST};
use diesel::prelude::*;
use serde_json::{json, Value};
use crate::schema::users::dsl::*;


use crate::utils::genrate_token;
use crate::{db::DbPool, models::user::User};

pub async fn signup(Json(user):Json<User>,Extension(pool): Extension<Arc<DbPool>>)->Result<impl IntoResponse, StatusCode>{
    let mut conn = pool.get().expect("Failed to get database connection from pool");
    
    // check whether the user already exists
    tracing::debug!("user_exist: {:?}", user);

    let existing_user = users.filter(email.eq(&user.email))
        .first::<User>(&mut conn)
        .optional()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    tracing::debug!("user_exist: {:?}", existing_user);

    // if user exists, return a 409 Conflict response
    if let Ok(Some(_)) = existing_user {
        return Ok((
            StatusCode::CONFLICT,
            Json(json!({"error":"User with that email already exists"}))
        ));
    }
    

    // else hash the password
    let hashed_password = bcrypt::hash(user.password, DEFAULT_COST).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);

    if let Err(_) = hashed_password {
        return Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error":"Failed to hash password"}))
        ));
    }

    // insert the user into the database
    let new_user=User{
        name:user.name.clone(),
        email: user.email.clone(),
        password: hashed_password.unwrap(),
    };

    let result = diesel::insert_into(users).values(&new_user).execute(&mut conn).map(|_| StatusCode::INTERNAL_SERVER_ERROR);

    if let Err(_) = result {
        return Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error":"Failed to insert user into database"}))
        ));
    }

    // generate a JWT token
    let token=genrate_token(new_user.email.to_string());

    if let Err(_) = token {
        return Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error":"Failed to generate token"}))
        ));
    }

    // return a 201 Created response
    Ok((
        StatusCode::CREATED,
        Json(json!({"message":"User created successfully","user":new_user,"token":token.unwrap()}))
    ))
}

pub async fn login(Json(user): Json<Value>,Extension(pool):Extension<Arc<DbPool>>)->Result<impl IntoResponse, StatusCode>{

    let mut conn = pool.get().expect("Failed to get database connection from pool");

    let user_email = match user.get("email") {
        Some(e) => e.as_str().ok_or(StatusCode::BAD_REQUEST)?,
        None => return Ok((StatusCode::BAD_REQUEST, Json(json!({"error":"Email is required"})))),
    };

    let user_password = match user.get("password") {
        Some(p)=>p.as_str().ok_or(StatusCode::BAD_REQUEST)?,
        None=> return Ok((StatusCode::BAD_REQUEST, Json(json!({"error":"Password is required"})))
        ),
    };

    tracing::debug!("email: {:?}", user_email);
    tracing::debug!("password: {:?}", user_password);

    let existing_user = match users.filter(email.eq(user_email))
        .first::<User>(&mut conn)
        .optional()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR) {
            Ok(Some(user)) => user,
            Ok(None) => return Ok((StatusCode::UNAUTHORIZED, Json(json!({"error":"User not found"})))),
            _ => return Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error":"Failed to get user"})))),
        };
    tracing::debug!("user_exist: {:?}", existing_user);

    // decrypt the password
    let is_password_valid=verify(user_password, existing_user.password.as_str()).map_err(|_|StatusCode::INTERNAL_SERVER_ERROR);

    if let Err(_) = is_password_valid {
        return Ok((StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error":"Failed to verify password"}))
        ));
        
    }

    if is_password_valid.unwrap()==false {
        return Ok((StatusCode::UNAUTHORIZED,
            Json(json!({"error":"Invalid password"}))
        ));
    }

    let token=genrate_token(existing_user.email.to_string());

    if let Err(_) = token {
        return Ok((StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error":"Failed to generate token"}))
        ));
    }

    Ok((StatusCode::OK,
        Json(json!({"message":"User logged in successfully","user":existing_user,"token":token.unwrap()}))))
}
