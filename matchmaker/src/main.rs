#[macro_use]
extern crate rocket;

mod key_generation;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use rocket_db_pools::deadpool_redis::{self, redis};
use rocket_db_pools::{Connection, Database};
use serde::{Deserialize, Serialize};
use serde_redis::RedisDeserialize;
use shared_models::Lobby;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct LobbyCreation {
    pub name: String,
    pub host: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct PlayerCountSettings {
    pub count: u8,
    pub secret: String,
}


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
async fn create_lobby(lobby: Json<LobbyCreation>, mut db: Connection<Lobbies>) -> Status {
    let lobby = lobby.0;
    if query_lobby(&lobby.name, &mut db).await.is_some() {
        return Status::Conflict;
    }

    let hash_name = format!("matchmaker/lobby:{}", lobby.name);

    let _: () = db
        .hset_multiple(
            &hash_name,
            &[
                ("name", lobby.name),
                ("playing", false.to_string()),
                ("player_count", 0.to_string()),
            ],
        )
        .await
        .unwrap();

    let _: () = db.sadd("matchmaker/lobbies", &hash_name).await.unwrap();
    Status::Ok
}

#[put(
    "/lobbies/<lobby>/playercount",
    format = "json",
    data = "<player_count_settings>"
)]
async fn set_player_count(
    lobby: String,
    player_count_settings: Json<PlayerCountSettings>,
    mut db: Connection<Lobbies>,
) -> Status {
    let player_count_settings = player_count_settings.0;
    if player_count_settings.secret != "secret" {
        return Status::Unauthorized;
    }
    let lobby = format!("matchmaker/lobby:{}", lobby);
    let _: () = db
        .hset(
            lobby,
            "player_count",
            player_count_settings.count.to_string(),
        )
        .await
        .unwrap();
    Status::Ok
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(Lobbies::init()).mount(
        "/",
        routes![list_lobbies, create_lobby, get_lobby, set_player_count],
    )
}
