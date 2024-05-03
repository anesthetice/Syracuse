use serde::{Deserialize, Serialize};
use std::{io::{Read, Write}, sync::OnceLock};
use crossterm::style::Stylize;

use crate::{animation::AnimationBuilder, warn};

pub static CONFIG: OnceLock<Config> = OnceLock::new(); 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    // should info statements be printed
    pub debug: bool,
    // what set of characters separate the names of an entry stored as a file
    pub entry_file_name_separtor: String,
    // how often should progress be automatically saved in seconds
    pub autosave_period: u16,
    // local utc offset to get accurate dates [HH, MM, SS]
    // e.g. western europe : [1,0,0] or [2,0,0] generally depending on daylight saving time
    // you will have to manually change the config to account for changes in your timezone
    pub local_offset: [i8; 3],
    // threshold for results to be considered
    pub search_threshold: f64,
    // smith-waterman and needlman-wunsch algorithm weight
    pub sw_nw_ratio: f64,
    // used for sw and nw algorithms
    pub match_score: i16,
    // used for sw and nw algorithms
    pub mismatch_penalty: i16,
    // used for sw and nw algorithms
    pub gap_penalty: i16,
    // approximately how long a frame will be displayed in milliseconds before being refreshed
    pub frame_period: u64,
    // don't ask me why this should be in a config file
    pub animation: AnimationBuilder
}

impl Default for Config {
    fn default() -> Self {
        Self {
            debug: false,
            entry_file_name_separtor: "-·-".to_string(),
            autosave_period: 30,
            local_offset: [0, 0, 0],
            search_threshold: 0.0,
            sw_nw_ratio: 0.6,
            match_score: 2,
            mismatch_penalty: -1,
            gap_penalty: -1,
            frame_period: 150,
            animation: vec![
                ("|  ".to_string(), "  |".to_string()),
                ("/  ".to_string(), "  /".to_string()),
                ("-  ".to_string(), "  -".to_string()),
                ("\\  ".to_string(), "  \\".to_string()),
            ],
        }
    }
}

impl Config {
    pub fn get() -> &'static Self {
        CONFIG.get().unwrap()
    }
    pub fn load(filepath: &std::path::Path) -> Self {
        match Self::from_file(filepath) {
            Ok(config) => config,
            Err(error) => {
                warn!("failed to load configuration from file, caused by : {}", error);
                let config = Self::default();
                if let Ok(downcast_error) = error.downcast::<std::io::Error>() {
                    if downcast_error.kind() == std::io::ErrorKind::NotFound {
                        match config.to_file(filepath) {
                            Ok(()) => warn!("created default configuration file, at : {}", filepath.display()),
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
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(filepath)?;

        file.write_all(&serde_json::to_vec_pretty(&self)?)?;
        file.flush()?;
        Ok(())
    }
}
