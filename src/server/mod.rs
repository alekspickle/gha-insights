use rocket::{catchers, launch, routes};
use rocket::{Build, Rocket};

mod catch;
mod routes;

#[launch]
pub fn rocket() -> Rocket<Build> {
    rocket::build()
        // .mount("/", routes![hello, hello]) // uncomment this to get an error
        // .mount("/", routes![unmanaged]) // uncomment this to get a sentinel error
        .mount("/", routes![routes::jobs, routes::repos])
        .register(
            "/",
            catchers![catch::general_not_found, catch::default_catcher],
        )
        .register("/current_jobs", catchers![catch::general_not_found])
}
