#[macro_use]
extern crate rocket;
use rocket_db_pools::Database;

mod client_api;
mod server_connection;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(client_api::Lobbies::init())
        .mount("/", client_api::get_routes())
}
