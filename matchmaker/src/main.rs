#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use rocket_db_pools::deadpool_redis::{self, redis};
use rocket_db_pools::{Connection, Database};
use serde_redis::RedisDeserialize;
use shared_models::Lobby;

#[derive(Database)]
#[database("lobbies")]
struct Lobbies(deadpool_redis::Pool);

#[get("/lobbies")]
async fn list_lobbies(mut db: Connection<Lobbies>) -> Json<Vec<Lobby>> {
    let lobby_names: Vec<String> = db.smembers("matchmaker/lobbies").await.unwrap();

    let mut lobbies = Vec::new();
    for lobby_name in lobby_names {
        let lobby_value: redis::Value = db.hgetall(&lobby_name).await.unwrap();
        let lobby: Lobby = lobby_value.deserialize().unwrap();
        lobbies.push(lobby);
    }
    Json(lobbies)
}

#[get("/lobbies/<lobby>")]
async fn get_lobby(lobby: String, mut db: Connection<Lobbies>) -> Json<Option<Lobby>> {
    let lobby = query_lobby(&lobby, &mut db).await;
    Json(lobby)
}

async fn query_lobby(lobby: &str, db: &mut Connection<Lobbies>) -> Option<Lobby> {
    let lobby = format!("matchmaker/lobby:{}", lobby);
    let lobby_value: redis::Value = db.hgetall(&lobby).await.unwrap();
    lobby_value.deserialize().ok()
}

#[put("/lobbies", format = "json", data = "<lobby>")]
async fn create_lobby(lobby: Json<Lobby>, mut db: Connection<Lobbies>) -> Status {
    if query_lobby(&lobby.name, &mut db).await.is_some() {
        return Status::Conflict;
    }

    let lobby = lobby.0;
    let hash_name = format!("matchmaker/lobby:{}", lobby.name);

    let _: () = db
        .hset_multiple(
            &hash_name,
            &[
                ("name", lobby.name),
                ("host", lobby.host),
                ("playing", lobby.playing.to_string()),
                ("player_count", lobby.player_count.to_string()),
            ],
        )
        .await
        .unwrap();

    let _: () = db.sadd("matchmaker/lobbies", &hash_name).await.unwrap();
    Status::Ok
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Lobbies::init())
        .mount("/", routes![list_lobbies, create_lobby, get_lobby])
}
