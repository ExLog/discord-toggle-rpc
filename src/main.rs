use discord_toggle_rpc::run;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run(env::args()).await;
    Ok(())
}
