use rocket::{Build, Rocket};

pub mod catch;
pub mod db;
pub mod routes;

#[rocket::launch]
pub fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", rocket::routes![routes::jobs, routes::repos])
        .register(
            "/",
            rocket::catchers![catch::general_not_found, catch::default_catcher],
        )
}
