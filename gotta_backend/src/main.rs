use axum::{
    routing::get,
    Router,
    Json
};

use serde::{Serialize, Deserialize};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/task", get(get_task).post(post_task).put(put_task))
        .route("/tasks", get(get_tasks));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

//TODO: figure out database setup for this and the database itself

// Todo Backend
// Get, Post, Put Task
// Get Tasks

#[derive(Serialize)]
struct Task {
    // owner_id: usize,
    name: String,
    is_completed: bool,
}

#[derive(Deserialize, Debug)]
struct CreateTask {
    name: String,
    is_completed: bool,
}

async fn get_task() -> Json<Task> {
    let fake_task = Task { name: "Homework".to_string(), is_completed: false};
    Json(fake_task)
}

async fn post_task(Json(payload): Json<CreateTask>) {
    println!("recieved a {:?}", payload);
}
async fn put_task() {}

async fn get_tasks() {}
