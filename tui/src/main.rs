pub mod action;
pub mod api_client;
pub mod app;
pub mod cli;
pub mod components;
pub mod config;
pub mod mode;
pub mod shared;
pub mod tui;

use api_client::ApiClient;
use clap::Parser;
use cli::Cli;
use color_eyre::eyre::Result;

use crate::{
    app::App,
    shared::utils::{initialize_logging, initialize_panic_handler},
};

async fn tokio_main() -> Result<()> {
    initialize_logging()?;

    initialize_panic_handler()?;

    let args = Cli::parse();

    let api_base_url =
        std::env::var("API_BASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    let api_client = ApiClient::new(api_base_url);

    let mut app = App::new(args.tick_rate, args.frame_rate, &api_client).await?;
    app.run().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(e) = tokio_main().await {
        eprintln!("{} error: Something went wrong", env!("CARGO_PKG_NAME"));
        Err(e)
    } else {
        Ok(())
    }
}
