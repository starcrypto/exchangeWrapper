mod config;
use crate::config::Config;
use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let mut config = Config::parse();
    config.load()?;
    Ok(())
}
