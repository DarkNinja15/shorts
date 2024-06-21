use axum::{http::{Request, StatusCode}, middleware::Next, response::Response};

use crate::handlers::user::validate_token;

pub async fn auth_middleware<B>(mut request:Request<B>,next:Next<B>)->Result<Response,StatusCode>{
    let headers=request.headers();
    let auth_token=headers.get("Authorization").ok_or(StatusCode::UNAUTHORIZED)?;

    
    let auth_token=auth_token.to_str().map_err(|_|StatusCode::UNAUTHORIZED)?
        .split_whitespace().nth(1).ok_or(StatusCode::UNAUTHORIZED)?;

    tracing::debug!("auth_token: {:?}", auth_token);

    let is_valid=validate_token(auth_token).map_err(|_|StatusCode::UNAUTHORIZED)?;

    request.extensions_mut().insert(is_valid);

    Ok(next.run(request).await)
}