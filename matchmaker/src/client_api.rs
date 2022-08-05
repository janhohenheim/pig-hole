use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::Route;
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use rocket_db_pools::deadpool_redis::{self, redis};
use rocket_db_pools::{Connection, Database};

use matchmaker_models::client_api::*;
use serde_redis::RedisDeserialize;

use crate::server_connection::create_client_connection_data;

#[derive(Database)]
#[database("lobbies")]
pub struct Lobbies(deadpool_redis::Pool);

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
    let lobby_value: redis::Value = db.hgetall(&lobby).await.unwrap();
    lobby_value.deserialize().ok()
}

#[put("/lobbies", format = "json", data = "<lobby>")]
async fn create_lobby(
    lobby: Json<LobbyCreation>,
    mut db: Connection<Lobbies>,
) -> Result<Json<LobbyResponse>, Status> {
    let lobby = lobby.0;
    if query_lobby(&lobby.name, &mut db).await.is_some() {
        return Err(Status::Conflict);
    }

    let hash_name = get_hash_name(&lobby.name);

    let _: () = db
        .hset_multiple(
            &hash_name,
            &[
                ("name", lobby.name.clone()),
                ("playing", false.to_string()),
                ("player_count", 0.to_string()),
            ],
        )
        .await
        .unwrap();

    let _: () = db.sadd("matchmaker/lobbies", &hash_name).await.unwrap();
    let response = create_client_connection_data(&lobby.name, &lobby.host);
    Ok(response.into())
}

#[put("/lobbies/<lobby>", format = "json", data = "<username>")]
async fn join_lobby(lobby: String, username: Json<String>) -> Json<LobbyResponse> {
    // Setting the player count is the job of the server now.
    create_client_connection_data(&lobby, &username.0).into()
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
            &lobby,
            "player_count",
            player_count_settings.count.to_string(),
        )
        .await
        .unwrap();
    if player_count_settings.count == 0 {
        delete_lobby(&lobby, &mut db).await;
    }
    Status::Ok
}

async fn delete_lobby(lobby: &str, db: &mut Connection<Lobbies>) {
    let lobby = get_hash_name(lobby);
    let _: () = db.del(&lobby).await.unwrap();
    let _: () = db.srem("matchmaker/lobbies", &lobby).await.unwrap();
}

fn get_hash_name(lobby: &str) -> String {
    format!("matchmaker/lobby:{}", lobby)
}

pub(crate) fn get_routes() -> Vec<Route> {
    routes![
        list_lobbies,
        create_lobby,
        get_lobby,
        set_player_count,
        join_lobby
    ]
}
