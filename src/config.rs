use anyhow::Result;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone, Eq)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
}

impl LogLevel {
    pub fn to_level_filter(self) -> log::LevelFilter {
        match self {
            LogLevel::Error => log::LevelFilter::Error,
            LogLevel::Warning => log::LevelFilter::Warn,
            LogLevel::Info => log::LevelFilter::Info,
            LogLevel::Debug => log::LevelFilter::Debug,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ExchangeSetting {
    pub rest_dsl_path: String,
    pub ws_dsl_path: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct InnerConfig {
    pub exchange_map: HashMap<String, Vec<ExchangeSetting>>,
    pub bind_addr: Option<String>,
    pub port: u16,
    pub log_path: Option<String>,
    pub log_level: LogLevel,
}

impl Default for InnerConfig {
    fn default() -> Self {
        Self {
            exchange_map: HashMap::new(),
            bind_addr: Some("127.0.0.1".to_string()),
            port: 50051,
            log_path: Some("./test.log".to_string()),
            log_level: LogLevel::Info,
        }
    }
}

// outer config structure. Used to define the parameter input/env input of the whole program
#[derive(Serialize, Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[arg(short, long, default_value_t=String::from("./config/config.yaml"))]
    pub config_path: String,
    #[arg(skip)]
    pub inner: InnerConfig,
}

impl Config {
    pub fn load(&mut self) -> Result<()> {
        let f = File::open(&self.config_path)?;
        self.inner = serde_yaml::from_reader(f)?;
        Ok(())
    }
}
