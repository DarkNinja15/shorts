use std::env;
use std::sync::Arc;

use axum::http::StatusCode;
use axum::{Extension, Json};
use bcrypt::DEFAULT_COST;
use diesel::prelude::*;
use jsonwebtoken::errors::Error;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::schema::users::dsl::*;

use crate::{db::DbPool, models::user::User};

use jsonwebtoken::{encode, Header, EncodingKey};

pub async fn signup(Json(user):Json<User>,Extension(pool): Extension<Arc<DbPool>>)->Result<impl axum::response::IntoResponse, StatusCode>{
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

pub async fn login(){}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
}

fn genrate_token(user_id:String)->Result<String,Error>{
    let claims=Claims{sub:user_id};

    let secret=env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    );

    token
}