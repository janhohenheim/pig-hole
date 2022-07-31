#[macro_use]
extern crate rocket;
use rocket_db_pools::Database;

mod client_api;
mod server_connection;
mod headers;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(client_api::Lobbies::init())
        .attach(headers::get_cors_fairing())
        .mount("/", client_api::get_routes())
}
