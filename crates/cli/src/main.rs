mod api;
mod cli;
mod commands;
mod config;
mod output;
mod ui;
mod util;

use anyhow::Result;
use clap::Parser;

use crate::cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    commands::run(cli.command).await
}
