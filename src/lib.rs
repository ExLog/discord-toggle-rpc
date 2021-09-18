extern crate dotenv;
use dotenv::dotenv;
use serde_json;
use std::{collections::HashMap, env, error::Error, io};

const SETTING_ENDPOINTS: &'static str = "https://discord.com/api/v9/users/@me/settings";

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
    }
}

pub async fn toggle_game_rpc(token: String) -> Result<serde_json::Value, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let res = client
        .get(SETTING_ENDPOINTS)
        .header("authorization", &token)
        .send()
        .await?;

    if res.status() != 200 {
        let bad_status_error = io::Error::new(
            io::ErrorKind::Other,
            format!("Bad status code: {}", res.status()),
        );
        return Err(Box::new(bad_status_error));
    }

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

    if res.status() == 200 {
        let current_settings: serde_json::Value = serde_json::from_str(&res.text().await?)?;

        return Ok(current_settings["show_current_game"].clone());
    } else {
        let bad_status_error = io::Error::new(
            io::ErrorKind::Other,
            format!("Bad status code: {}", res.status()),
        );

        return Err(Box::new(bad_status_error));
    }
}
