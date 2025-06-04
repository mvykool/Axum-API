use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use axum::{
    extract::Extension,
    routing::get,
    Json,
    Router
};
use tracing::{info, Level};
use tracing_subscriber; 

//main post struct
#[derive(Serialize, Deserialize)]
struct Post {
    id: i32,
    user_id: Option<i32>,
    title: String,
    body: String,
}

// create post struct
#[derive(Serialize, Deserialize)]
struct CreatePost {
    title: String,
    body: String,
    user_id: Option<i32>,
}

// create update struct
#[derive(Serialize, Deserialize)]
struct UpdatePost {
    title: String,
    body: String,
    user_id: Option<i32>,
}

// create message struct
#[derive(Serialize)]
struct Message {
    message: String,
}

// user structs
#[derive(Serialize, Deserialize)]
struct CreateUser {
    username: String,
    email: String
}

#[derive(Serialize, Deserialize)]
struct User {
    id: i32,
    username: String,
    email: String
}

// handle user with POST request 
async fn create_user(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(new_user): Json<CreateUser>,
) -> Result<Json<User>, StatusCode> {
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (username, email) VALUES ($1, $2) RETURNING id, username, email",
        new_user.username,
        new_user.email
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(user))
}

// handler for get all posts
async fn get_posts(
    Extension(pool): Extension<Pool<Postgres>>
) -> Result<Json<Vec<Post>>, StatusCode> {
    let posts = sqlx::query_as!(Post, "SELECT id, title, body FROM posts")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(posts))
}

//create POST handler, which will have the same endpoint of /posts/
async fn create_post(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(new_post): Json<CreatePost>,
) -> Result<Json<Post>, StatusCode>{
    let post = sqlx::query_as!(
        Post,
        "INSERT INTO posts (user_id, title, body) VALUES ($1, $2, $3) RETURNING id, title, body, user_id",
        new_post.user_id,
        new_post.title,
        new_post.body
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(post))
}

// creating update handler, which will have the same endpoint of posts/:id 
async fn update_post(
    Extension(pool): Extension<Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(updated_post): Json<UpdatePost>,
) -> Result<Json<Post>, StatusCode>{
    let post = sqlx::query_as!(
        Post,
        "UPDATE posts SET title = $1, body = $2, user_id = $3 WHERE id = $4 RETURNING id, user_id, title, body"
        updated_post.title,
        updated_post.body,
        updated_post.user_id,
        id
    )
    .fetch_one(&pool)
    .await;

    match post{
        Ok(post) => Ok(Json(post)),
        Error(_) => Error(StatusCode::NOT_FOUND),
    }
}

// create delete handler,it should be the same endpoint as posts/:id
async fn delete_post(
    Extension(pool): Extension<Pool<Postgres>>,
    Path(id):Path<i32>,
) -> Result<Json<serde_json::Value>, StatusCode>{
    let result = sqlx::query!("DELETE FROM posts WHERE id = $1", id)
        .execute(&pool)
        .await

    match result {
        Ok(_) => Ok(Json(serde_json::json! ({
            "message": "Post deleted successfully"
        }))),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }

}

// handler for get one post
async fn get_one_post(
    Extension(pool): Extension<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<Json<Post>, StatusCode> {
    let one_post = sqlx::query_as!(
        Post,
        "SELECT id, user_id, title, body FROM posts WHERE id = $1",
        id
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(one_post))
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
        //endpoint for users
        .route("/users", post(create_user));
        // the posts endpoint will have both a GET request, and POST request
        .route("/posts", get(get_posts).post(create_post));
        .route("/posts/:id", get(get_one_post).put(update_post).delete(delete_post));
        // axum extension layer
        .layer(Extension(pool));

    //run app with hyper that listening globally to our port
    //run server on 5001, 5000 is busy
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5001").await.unwrap();
    info!("server running on http://0.0.0.0:5001");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

