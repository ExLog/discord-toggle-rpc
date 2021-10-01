extern crate dotenv;
mod utils;

use dotenv::dotenv;
use serde_json::from_str;
use std::{collections::HashMap, env, error::Error, io};
use utils::handle_status_error;

const SETTING_ENDPOINTS: &str = "https://discord.com/api/v9/users/@me/settings";
const CONNECTION_ENDPOINTS: &str = "https://discord.com/api/v9/users/@me/connections";

pub async fn run(mut args: env::Args) {
    dotenv().ok();
    let discord_token = env::var("DISCORD_TOKEN").expect("Discord Token is not found");

    // skip the first index (name of binary)
    args.next();

    match args.next() {
        Some(arg) => {
            match_arg(arg, discord_token).await;
        }
        None => {
            eprintln!("Please include arguments.");
        }
    };
}

pub async fn match_arg(arg: String, token: String) {
    if arg == "game" {
        match toggle_game_rpc(token).await {
            Ok(status) => {
                println!("Show playing game: {}", status);
            }
            Err(err) => {
                eprintln!("An error occured: {}", err);
            }
        }
    } else if arg == "spotify" {
        match toggle_spotify_rpc(token).await {
            Ok(status) => {
                println!("Show listening Spotify: {}", status);
            }
            Err(err) => {
                eprintln!("An error occured: {}", err);
            }
        }
    }
}

pub async fn toggle_spotify_rpc(token: String) -> Result<serde_json::Value, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let res = client
        .get(CONNECTION_ENDPOINTS)
        .header("authorization", &token)
        .send()
        .await?;

    handle_status_error(res.status().as_u16())?;

    let connections: serde_json::Value = from_str(&res.text().await?)?;

    let mut spotify_id: Option<&str> = None;
    let mut show_activity_spotify: Option<bool> = None;

    for connection in connections.as_array().unwrap().iter() {
        if connection["type"].as_str().unwrap() == "spotify" {
            spotify_id = connection["id"].as_str();
            show_activity_spotify = connection["show_activity"].as_bool();
            break;
        }
    }

    // Check whether spotify_id is None or not
    if spotify_id.is_none() {
        let connection_error =
            io::Error::new(io::ErrorKind::Other, "Account is not connected to Spotify");

        return Err(Box::new(connection_error));
    };

    let spotify_id = spotify_id.unwrap();
    let show_activity_spotify = show_activity_spotify.unwrap();

    let mut body = HashMap::new();

    body.insert("show_activity", !show_activity_spotify);

    let res = client
        .patch(format!("{}/spotify/{}", CONNECTION_ENDPOINTS, spotify_id))
        .header("authorization", &token)
        .json(&body)
        .send()
        .await?;

    if let Err(err) = handle_status_error(res.status().as_u16()) {
        Err(err)
    } else {
        let current_settings: serde_json::Value = serde_json::from_str(&res.text().await?)?;

        Ok(current_settings["show_activity"].clone())
    }
}

pub async fn toggle_game_rpc(token: String) -> Result<serde_json::Value, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let res = client
        .get(SETTING_ENDPOINTS)
        .header("authorization", &token)
        .send()
        .await?;

    handle_status_error(res.status().as_u16())?;

    // parse from json
    let settings: serde_json::Value = serde_json::from_str(&res.text().await?)?;

    let mut body = HashMap::new();
    let show_current_game = &settings["show_current_game"];

    body.insert("show_current_game", !show_current_game.as_bool().unwrap());

    let res = client
        .patch(SETTING_ENDPOINTS)
        .header("authorization", &token)
        .json(&body)
        .send()
        .await?;

    if let Err(err) = handle_status_error(res.status().as_u16()) {
        Err(err)
    } else {
        let current_settings: serde_json::Value = serde_json::from_str(&res.text().await?)?;

        Ok(current_settings["show_current_game"].clone())
    }
}

#[tokio::test]
#[should_panic(expected = "Unauthorized request")]
async fn unauthorized_request_game() {
    let discord_token = String::from("Invalid token peko");

    match toggle_game_rpc(discord_token).await {
        Ok(_) => {
            // Should not run
            panic!("Oh no.");
        }
        Err(_) => {
            panic!("Unauthorized request");
        }
    }
}

#[tokio::test]
#[should_panic(expected = "Unauthorized request")]
async fn unauthorized_request_spotify() {
    let discord_token = String::from("Invalid token peko");

    match toggle_spotify_rpc(discord_token).await {
        Ok(_) => {
            // Should not run
            panic!("Oh no.");
        }
        Err(_) => {
            panic!("Unauthorized request");
        }
    }
}
