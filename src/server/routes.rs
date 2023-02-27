use crate::{server::db::{queries, Run}, JobInfo};
use rocket::{get, post};

#[get("/<repo>/jobs/<search>")]
pub(crate) fn jobs(repo: &str, search: &str) -> Result<Vec<JobInfo>> {
    sqlx::query(&queries::jobs::create_table())?;

    let s = sqlx::query(&queries::create_user())
        .bind(account.car)
        .bind(&account.email)
        .bind(account.bank_details)
        .bind(hashed)
        .execute(&pool)
        .await
        .map_err(internal_error)?;

    format!("Searching for jobs in repo {repo}: {search}")
}

#[get("/<repo>/runs/<search>")]
pub(crate) fn runs(repo: &str, search: &str) -> String {
    format!("Searching for runs in repo {repo}: {search}")
}

//#[post("/", data = "<post>")]
//async fn create(db: Db, post: Json<Run>) -> Result<Created<Json<Run>>> {
//    let post_value = post.clone();
//    db.run(move |conn| {
//        diesel::insert_into(runs::table)
//            .values(&*post_value)
//            .execute(conn)
//    })
//    .await?;
//
//    Ok(Created::new("/").body(post))
//}
