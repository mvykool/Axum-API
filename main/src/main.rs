use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use axum::{
    routing::get,
    Router,
};
use tracing::{info, Level};
use tracing_subscriber; 

#[derive(Serialize, Deserialize)]
struct Post {
    id: i32,
    user_id: Option<i32>,
    title: String,
    body: String,
}


// handler for GET
async fn root() -> &'static str {
    "hello, world"
}

// handler for get posts
async fn get_posts(
    Extension(pool): Extension<Pool<Postgres>>
) -> Result<Json<Vec<Post>>, StatusCode> {
    let posts = sqlx::query_as!(Post, "SELECT id, title, body FROM posts")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(posts))
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    //let's initializa the tracing, to log info
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();
    
    dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new().connect(&url).await?;
    println!("connect to database");

    // build application with router
    let app = Router::new()
        // get goes to root
        .route("/", get(root));
        // axum extension layer
        .layer(Extension(pool));

    //run app with hyper that listening globally to our port
    //run server on 5001, 5000 is busy
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5001").await.unwrap();
    info!("server running on http://0.0.0.0:5001");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

