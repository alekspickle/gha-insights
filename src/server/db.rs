use rocket::fairing::{self, AdHoc};
use rocket::response::status::Created;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{futures, Build, Rocket};

use rocket_db_pools::{sqlx, Connection, Database, Pool};

use futures::{future::TryFutureExt, stream::TryStreamExt};

#[derive(Database)]
#[database("sqlx")]
struct Db(Pool);

pub(crate) const RUNS_TABLE_NAME: &str = "runs";
pub(crate) const JOBS_TABLE_NAME: &str = "jobs";
pub(crate) const STEPS_TABLE_NAME: &str = "steps";

pub type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

pub mod queries {
 
    /// Queries related to jobs
    pub mod runs {
        pub(crate) fn create_table() -> String {
            format!(
                "
            CREATE TABLE IF NOT EXISTS {} (
                id BIGINT PRIMARY KEY,
                workflow_id BIGINT,
                commit_sha TEXT,
                status TEXT 
            )
        ",
                super::RUNS_TABLE_NAME
            )
        }

        pub(crate) fn insert() -> String {
            format!(
                "INSERT INTO {} (id, name, status) VALUES ($1, $2, $3)",
                super::RUNS_TABLE_NAME,
            )
        }

        pub(crate) fn get_all() -> String {
            format!("SELECT name from {}", super::RUNS_TABLE_NAME,)
        }
    }

    /// Queries related to jobs
    pub mod jobs {
        pub(crate) fn create_table() -> String {
            format!(
                "
            CREATE TABLE IF NOT EXISTS {} (
                id BIGINT PRIMARY KEY,
                run_id BIGINT REFERENCES runs(id),
                name TEXT,
                status TEXT
            )
        ",
                super::JOBS_TABLE_NAME
            )
        }

        pub(crate) fn insert() -> String {
            format!(
                "INSERT INTO {} (id, name, status) VALUES ($1, $2, $3)",
                super::JOBS_TABLE_NAME,
            )
        }

        pub(crate) fn get_all() -> String {
            format!(
                "SELECT name from {} WHERE run_id = $1",
                super::JOBS_TABLE_NAME,
            )
        }
    }

    /// Queries related to steps
    pub mod steps {
        pub(crate) fn create_table() -> String {
            format!(
                "
            CREATE TABLE IF NOT EXISTS {} (
                id BIGINT PRIMARY KEY,
                job_id BIGINT REFERENCES jobs(id),
                name TEXT,
                status TEXT
            )
        ",
                super::STEPS_TABLE_NAME
            )
        }

        pub(crate) fn insert() -> String {
            format!(
                "INSERT INTO {} (id, name, status) VALUES ($1, $2, $3)",
                super::STEPS_TABLE_NAME,
            )
        }

        pub(crate) fn get_all() -> String {
            format!(
                "SELECT name from {} WHERE job_id = $1",
                super::STEPS_TABLE_NAME,
            )
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Run {
    id: i64,
    workflow_id: i64,
    commit_sha: String,
    status: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Job {
    id: i64,
    run_id: i64,
    name: i64,
    status: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Step {
    id: i64,
    job_id: i64,
    name: i64,
    status: String,
}

/* #[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Post {
#[serde(skip_deserializing, skip_serializing_if = "Option::is_none")]
id: Option<i64>,
title: String,
text: String,
} */

//#[post("/", data = "<post>")]
//async fn create(mut db: Connection<Db>, post: Json<Post>) -> Result<Created<Json<Post>>> {
//    // There is no support for `RETURNING`.
//    sqlx::query!("INSERT INTO posts (title, text) VALUES (?, ?)", post.title, post.text)
//        .execute(&mut *db)
//        .await?;
//
//    Ok(Created::new("/").body(post))
//}
//
//#[get("/")]
//async fn list(mut db: Connection<Db>) -> Result<Json<Vec<i64>>> {
//    let ids = sqlx::query!("SELECT id FROM posts")
//        .fetch(&mut *db)
//        .map_ok(|record| record.id)
//        .try_collect::<Vec<_>>()
//        .await?;
//
//    Ok(Json(ids))
//}
//
//#[get("/<id>")]
//async fn read(mut db: Connection<Db>, id: i64) -> Option<Json<Post>> {
//    sqlx::query!("SELECT id, title, text FROM posts WHERE id = ?", id)
//        .fetch_one(&mut *db)
//        .map_ok(|r| Json(Post { id: Some(r.id), title: r.title, text: r.text }))
//        .await
//        .ok()
//}
//
//#[delete("/<id>")]
//async fn delete(mut db: Connection<Db>, id: i64) -> Result<Option<()>> {
//    let result = sqlx::query!("DELETE FROM posts WHERE id = ?", id)
//        .execute(&mut *db)
//        .await?;
//
//    Ok((result.rows_affected() == 1).then(|| ()))
//}
//
//#[delete("/")]
//async fn destroy(mut db: Connection<Db>) -> Result<()> {
//    sqlx::query!("DELETE FROM posts").execute(&mut *db).await?;
//
//    Ok(())
//}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match Db::fetch(&rocket) {
        Some(db) => match sqlx::migrate!("db/sqlx/migrations").run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        },
        None => Err(rocket),
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("SQLx Stage", |rocket| async {
        rocket
            .attach(Db::init())
            .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
            .mount("/sqlx", routes![list, create, read, delete, destroy])
    })
}
