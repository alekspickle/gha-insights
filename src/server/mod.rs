use rocket::{Build, Rocket};

mod catch;
mod routes;

#[rocket::launch]
pub fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", rocket::routes![routes::jobs, routes::repos])
        .register(
            "/",
            rocket::catchers![catch::general_not_found, catch::default_catcher],
        )
}
