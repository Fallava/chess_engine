mod bots;
mod engine;
mod api;

use crate::api::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let api = Api::new();

    api.loop_event().await?;

    Ok(())
}
