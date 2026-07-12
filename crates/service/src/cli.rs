use clap::Parser;

use crate::shared::{config::Config, error::Result};

/// Cli
#[derive(Debug, Parser)]
#[command(version, propagate_version = true)]
pub struct Cli {
    /// Path to the config file
    #[arg(short, long, env = "CONFIG_FILE")]
    pub config_file: Option<String>,
}

impl Cli {
    pub fn parse() -> Self {
        Parser::parse()
    }

    pub fn load_config() -> Result<Config> {
        let cli = Cli::parse();

        let config = match cli.config_file {
            Some(path) => Config::try_from_file(&path)?,
            None => Default::default(),
        };
        Ok(config)
    }
}
