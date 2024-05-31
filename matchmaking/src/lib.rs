use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use axum::{Extension, Json, Router};
use axum::http::StatusCode;
use axum::routing::{get, post, put};
use chrono;
use hmac::{Hmac, KeyInit, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use base64::prelude::*;
use network::buffer::Buffer;

type HmacSha256 = Hmac<Sha256>;

#[derive(Serialize, Deserialize, Debug)]
struct MatchmakerTokenPayload {
    #[serde(rename = "Puid")] player_uid: String,
    #[serde(rename = "ClientVersion")] client_version: String,
    #[serde(rename = "ExpiresAt")] expiration: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct MatchmakerToken {
    #[serde(rename = "Content")] content: MatchmakerTokenPayload,
    #[serde(rename = "Hash")] content_hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct HostServer {
    #[serde(rename = "Ip")] net_address: i32,
    #[serde(rename = "Port")] port: i16,
}

#[derive(Serialize, Deserialize, Debug)]
struct UserTokenRequestData {
    #[serde(rename = "Puid")] player_uid: String,
    #[serde(rename = "Username")] username: String,
    #[serde(rename = "ClientVersion")] client_version: String,
    #[serde(rename = "Language")] language: String,
}
#[derive(Clone, Debug)]
pub struct Context {
    pub private_key: [u8; 128]
}

pub async fn create_matchmaking_thread(private_key: [u8; 128]) {
    // println!("Heee");
    let app = Router::new()
        .route("/api/user", post(create_token))
        .route("/api/games", put(get_game_address))
        .route("/api/test", get(test))
        .layer(Extension(private_key));
    println!("Matchmaking server running on 127.0.0.1:22022");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:22022").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub async fn test() -> (StatusCode, String) {
    return (StatusCode::OK, "hi".to_string());
}

pub async fn get_game_address(Extension(private_key): Extension<[u8; 128]>) -> (StatusCode, String) {
    let address_bits = Buffer::from(vec![127,0,0,1]).read_int();
    let server = HostServer {
        net_address: address_bits,
        port: 22023
    };
    return (StatusCode::OK, serde_json::to_string(&server).unwrap());
}


pub async fn create_token(Extension(private_key): Extension<[u8; 128]>, Json(request): Json<UserTokenRequestData>) -> (StatusCode, String) {
    println!("Created token stuff");
    let payload = MatchmakerTokenPayload {
        player_uid: request.player_uid,
        client_version: request.client_version,
        expiration: chrono::offset::Local::now().to_rfc3339(),
    };

    let payload_string = serde_json::to_string(&payload).unwrap();
    let mut mac = HmacSha256::new_from_slice(&private_key).expect("Could not create HMAC instance");
    mac.update(payload_string.as_bytes());

    let payload_hash = mac.finalize();

    let token = MatchmakerToken {
        content: payload,
        content_hash: BASE64_STANDARD.encode(payload_hash.as_bytes()),
    };

    return (StatusCode::OK, BASE64_STANDARD.encode(serde_json::to_string(&token).unwrap().as_bytes()));
}

fn closure<F>(callback: F) -> ()
    where
        F: Send + Fn(Context) + 'static,
{
}