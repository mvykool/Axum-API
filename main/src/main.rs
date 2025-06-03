use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use axum::{
    routing::get,
    Router,
};
use tracing::{info, Level};
use tracing_subscriber;
 
#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    //let's initializa the tracing, to log info
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();
    
    dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let _pool = PgPoolOptions::new().connect(&url).await?;
    println!("connect to database");

    // build application with router
    let app = Router::new()
        // get goes to root
        .route("/", get(root));

    //run app with hyper that listening globally to our port
    //run server on 5001, 5000 is busy
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5001").await.unwrap();
    info!("server running on http://0.0.0.0:5001");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

// handler for GET
async fn root() -> &'static str {
    "hello, world"
}
