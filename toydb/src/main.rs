use axum::{
    extract::{Path, State},
    routing::{get, post},
    Router,
    response::{IntoResponse, Response},
    http::StatusCode,
};
use tokio::sync::RwLock;
use std::sync::Arc;
use crate::database::Database;

mod persistence;
mod database;

type Store = Arc<RwLock<Database>>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let store: Store = Arc::new(RwLock::new(Database::new("test.db".to_string())));

    let app = Router::new()
        .route("/set/{k}/{v}", post(store_set_value))
        .route("/get/{k}", get(store_get_value))
        .with_state(store);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

// -------- handlers ----------
#[derive(Debug)]
enum GetResult {
    Found(String),
    NotFound,
}

impl IntoResponse for GetResult {
    fn into_response(self) -> Response {
        match self {
            GetResult::Found(val) => val.into_response(), // 200 OK by default
            GetResult::NotFound => StatusCode::NOT_FOUND.into_response(),
        }
    }
}

async fn store_set_value(
    State(store): State<Store>,
    Path((k, v)): Path<(String, String)>,
) {
    store.write().await.put(k, v);
}

async fn store_get_value(
    State(store): State<Store>,
    Path(k): Path<String>,
) -> GetResult {
    let guard = store.read().await;
    match guard.get(&k) {
        Some(value) => GetResult::Found(value.clone()),
        None => GetResult::NotFound,
    }
}