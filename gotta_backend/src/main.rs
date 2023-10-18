use axum::{extract::Path, routing::post};
#[allow(unused)]
use axum::{
    extract::State,
    routing::{get, put},
    Json, Router,
};

use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let db: PgPool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://demouser:123@localhost:5432/gottadb")
        .await?;

    let app = Router::new()
        .route("/task", post(create_task))
        .route("/task/:id", get(get_task).put(update_task))
        .route("/tasks", get(get_tasks))
        .with_state(db);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

// Todo Backend
// GET /task/:id -> return a single task of id or error
// PUT /task/:id -> update a single task and return the task or error
// GET /tasks -> return all tasks or error
// POST /task -> create a new task and return the id or error

#[derive(Serialize, Debug)]
struct Task {
    id: i64,
    name: String,
    is_completed: bool,
}

#[derive(Deserialize, Debug)]
struct CreateTask {
    name: String,
    is_completed: bool,
}

#[derive(Deserialize, Debug)]
struct UpdateTask {
    name: String,
    is_completed: bool,
}

#[derive(Serialize, Debug)]
struct TaskID {
    id: i64,
}

impl TaskID {
    fn new(id: i64) -> Self {
        Self { id }
    }
}

async fn get_task(State(pool): State<PgPool>, Path(id): Path<i64>) -> Result<Json<Task>, String> {
    match sqlx::query_as!(Task, "SELECT * FROM task WHERE id = $1", id)
        .fetch_one(&pool)
        .await
    {
        Ok(tasks) => Ok(Json(tasks)),
        Err(_) => return Err("Cannot retrieve tasks from database".to_string()),
    }
}

// For the simplicity of this project all tasks belong to one user, the person locally running the app
async fn get_tasks(State(pool): State<PgPool>) -> Result<Json<Vec<Task>>, String> {
    println!("GETALLTASKS");
    match sqlx::query_as!(Task, "SELECT * FROM task")
        .fetch_all(&pool)
        .await
    {
        Ok(tasks) => Ok(Json(tasks)),
        Err(_) => return Err("Cannot retrieve tasks from database".to_string()),
    }
}

async fn create_task(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateTask>,
) -> Result<Json<TaskID>, String> {
    println!("CREATETASK: recieved {:?}", payload);
    match sqlx::query!(
        "
        INSERT INTO task (name, is_completed)
        VALUES ( $1, $2)
        RETURNING id
        ",
        payload.name,
        payload.is_completed
    )
    .fetch_one(&pool)
    .await
    {
        Ok(task) => Ok(Json(TaskID::new(task.id))),
        Err(_) => return Err("Cannot create task".to_string()),
    }
}

async fn update_task(
    State(pool): State<PgPool>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateTask>,
) -> Result<Json<Task>, String> {
    println!("UPDATETASK: recieved {:?}", payload);
    match sqlx::query_as!(
        Task,
        "UPDATE task 
                 SET name = $1, is_completed = $2
                 WHERE id = $3
                 RETURNING id, name, is_completed",
        payload.name,
        payload.is_completed,
        id
    )
    .fetch_one(&pool)
    .await
    {
        Ok(updated_task) => Ok(Json(updated_task)),
        Err(_) => Err("Cannot update task".to_string()),
    }
}
