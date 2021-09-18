extern crate dotenv;

use dotenv::dotenv;
use std::env;

use discord_toggle_rpc::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let discord_token = env::var("DISCORD_TOKEN").expect("Discord Token is not found");

    run(env::args(), discord_token).await;


    Ok(())
}
