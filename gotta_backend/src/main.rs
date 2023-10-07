use axum::routing::post;
#[allow(unused)]
use axum::{extract::State, routing::get, Json, Router};

use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let db: PgPool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://demouser:123@localhost:5432/gottadb")
        .await?;

    let app = Router::new()
        .route("/task", get(get_task).post(create_task).put(put_task))
        .route("/tasks", get(get_tasks))
        .with_state(db);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

//TODO: figure out database setup for this and the database itself

// Todo Backend
// Get, Post, Put Task
// Get Tasks

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

async fn get_task() -> Json<Task> {
    let fake_task = Task {
        id: 99,
        name: "Homework".to_string(),
        is_completed: false,
    };
    Json(fake_task)
}

async fn complete_task() {}

async fn create_task(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateTask>,
) -> Result<Json<i64>, String> {
    println!("CREATETASK: recieved {:?}", payload);
    let new_task = match sqlx::query!(
        r#"
        INSERT INTO task (name, is_completed)
        VALUES ( $1, $2)
        RETURNING id
        "#,
        payload.name,
        payload.is_completed
    )
    .fetch_one(&pool)
    .await
    {
        Ok(task) => task,
        Err(_) => {
            return Err("Cannot create task".to_string())
        }
    };

    Ok(Json(new_task.id))
}
async fn put_task() {}

async fn get_tasks() {
    // let rows: Vec<Task> = sqlx::query_as!(Task, r"SELECT * FROM task")
    //     .fetch_all(&db)
    //     .await?;
    // let all_tasks = rows.iter().map(|r| r.)
    // println!("{:?}", rows);
}
