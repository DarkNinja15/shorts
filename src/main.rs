use std::{net::SocketAddr, sync::Arc};

use news_shorts::{config, db, routes::create_routes};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main(){
    config::init();
    tracing_subscriber::fmt().with_env_filter(EnvFilter::from("debug")).init();

    let pool=Arc::new(db::establish_connection());
    let app=create_routes(pool);

    let addr=SocketAddr::from(([127,0,0,1],3000));
    tracing::debug!("listening on {}",addr);

    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}