use rocket::get;

#[get("/<repo>/jobs/<search>")]
pub(crate) fn jobs(repo: &str, search: &str) -> String {
    format!("Searching for jobs in repo {repo}: {search}")
}

#[get("/<repo>/runs/<search>")]
pub(crate) fn repos(repo: &str, search: &str) -> String {
    format!("Searching for runs in repo {repo}: {search}")
}
