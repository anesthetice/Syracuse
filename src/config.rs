use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use crossterm::style::Stylize;

use crate::{info, warn};

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
}

impl Default for Config {
    fn default() -> Self {
        Self {}
    }
}

impl Config {
    pub fn load(filepath: &std::path::Path) -> Self {
        match Self::from_file(filepath) {
            Ok(config) => config,
            Err(error) => {
                warn!("failed to load configuration from file, caused by : {}", error);
                let config = Self::default();
                if let Ok(downcast_error) = error.downcast::<std::io::Error>() {
                    if downcast_error.kind() == std::io::ErrorKind::NotFound {
                        match config.to_file(filepath) {
                            Ok(()) => info!("created default configuration file, at : {}", filepath.display()),
                            Err(error) => warn!("failed to create default configuration file, at : {}, caused by : {}", filepath.display(), error)
                        }
                    }
                }
                config
            }
        }
    }

    fn from_file(filepath: &std::path::Path) -> anyhow::Result<Self> {
        let mut buffer: Vec<u8> = Vec::new();
        std::fs::OpenOptions::new()
            .create(false)
            .read(true)
            .open(filepath)?
            .read_to_end(&mut buffer)?;
        Ok(serde_json::from_slice(&buffer)?)
    }

    fn to_file(&self, filepath: &std::path::Path) -> anyhow::Result<()> {
        Ok(std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(filepath)?
            .write_all(&serde_json::to_vec_pretty(&self)?)?)
    }
}
