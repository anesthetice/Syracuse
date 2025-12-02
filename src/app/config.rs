use crate::{animation::AnimationBuilder, app::SortOptions};
use color_eyre::Result;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Determines how often should progress be automatically saved in seconds.
    pub autosave_period: u16,
    // The default backup path.
    pub backup_path: String,
    /// Determines in which order entries are listed, with the start defined as the top, and the end as the bottom, the following values are possible: NameAscending, NameDescending, DurationAscending, DurationDescending.
    pub sort_option: SortOptions,
    /// Determines the numbers of hours past midnight for which running a command will count for the previous day.
    pub night_owl_hour_extension: i8,

    /// The threshold for results to be considered.
    pub search_threshold: f64,
    /// The relative weights of the 'Smith-Waterman' and 'Needlman-Wunsch' algorithms respectively.
    pub sw_nw_ratio: f64,

    /// Determines how long in milliseconds a frame will be displayed before being refreshed.
    pub frame_period: u64,
    /// The animation frames, an array containing (left, right) strings.
    pub animation: AnimationBuilder,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            autosave_period: 30,
            backup_path: "".to_string(),
            sort_option: SortOptions::default(),
            night_owl_hour_extension: 0,
            search_threshold: 0.0,
            sw_nw_ratio: 0.5,
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
    pub fn load(filepath: &std::path::Path) -> Self {
        match Self::from_file(filepath) {
            Ok(config) => config,
            Err(err) => {
                eprintln!("Warning: Failed to load configuration from file, '{err}'");
                let config = Self::default();
                let Ok(downcast_error) = err.downcast::<std::io::Error>() else {
                    return config;
                };
                if downcast_error.kind() == std::io::ErrorKind::NotFound {
                    match config.to_file(filepath) {
                        Ok(()) => eprintln!(
                            "Warning: Created default configuration file, at '{}'",
                            filepath.display()
                        ),
                        Err(error) => eprintln!(
                            "Warning: Failed to create default configuration file, at '{}', caused by '{}'",
                            filepath.display(),
                            error
                        ),
                    }
                }
                config
            }
        }
    }

    fn from_file(filepath: &std::path::Path) -> Result<Self> {
        let mut buffer: Vec<u8> = Vec::new();
        std::fs::OpenOptions::new()
            .create(false)
            .read(true)
            .open(filepath)?
            .read_to_end(&mut buffer)?;
        Ok(serde_json::from_slice(&buffer)?)
    }

    fn to_file(&self, filepath: &std::path::Path) -> Result<()> {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(filepath)?;
        file.write_all(&serde_json::to_vec_pretty(self)?)?;
        file.sync_all()?;
        Ok(())
    }
}
