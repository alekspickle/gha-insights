use rocket::{catchers, launch, routes};
use rocket::{Build, Rocket};

mod catch;
mod routes;

#[launch]
pub fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![routes::jobs, routes::repos])
        .register(
            "/",
            catchers![catch::general_not_found, catch::default_catcher],
        )
}
