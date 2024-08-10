use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    sync::OnceLock,
};

use crate::{
    animation::AnimationBuilder, data::graphing::interpolation::InterpolationMethod, warn,
};

pub static CONFIG: OnceLock<Config> = OnceLock::new();
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    // what set of characters separate the names of an entry stored as a file
    pub entry_file_name_separtor: String,
    // how often should progress be automatically saved in seconds
    pub autosave_period: u16,
    // default backup path
    pub backup_path: String,
    // when starting a stopwatch for a given entry, should the initial time be displayed?
    // realistically this could also just be an argument option in the CLI, but I personally want
    // it to be always on, so there we go..
    pub stopwatch_explicit: bool,
    // by how many hours should the day be extended after midnight
    // e.g. 2 -> timers started until 2 a.m. on a given day will count towards the previous day
    // useful for night owls
    pub night_owl_hour_extension: i8,

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
    pub animation: AnimationBuilder,

    // empty string means where directory from which syracuse was executed
    pub graph_output_dir: String,
    // "Linear" or "Makima" interpolation are currently available, note that Makima overshoots
    pub graph_interpolation_method: InterpolationMethod,
    // the number of points between a date and the next one that will be interpolated when graphing the sum of entries
    pub graph_nb_interpolated_points: usize,
    // marker size for entries
    pub graph_marker_size: u32,
    // graph background color
    pub graph_background_rgb: (u8, u8, u8),
    // graph foreground color
    pub graph_foreground_rgb: (u8, u8, u8),
    // graph bold grid color
    pub graph_coarse_grid_rgb: (u8, u8, u8),
    // graph fine grid color
    pub graph_fine_grid_rgb: (u8, u8, u8),
    // graph sum line color
    pub graph_sum_line_rgb: (u8, u8, u8),
    // the colors used for entry markers
    pub graph_marker_rgb: Vec<(u8, u8, u8)>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            entry_file_name_separtor: "-·-".to_string(),
            autosave_period: 30,
            backup_path: "".to_string(),
            stopwatch_explicit: false,
            night_owl_hour_extension: 0,
            search_threshold: 0.0,
            sw_nw_ratio: 0.5,
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
            graph_output_dir: "".to_string(),
            graph_interpolation_method: InterpolationMethod::Linear,
            graph_nb_interpolated_points: 1500,
            graph_marker_size: 6,
            graph_background_rgb: (30, 30, 46),
            graph_foreground_rgb: (205, 214, 244),
            graph_coarse_grid_rgb: (84, 87, 108),
            graph_fine_grid_rgb: (49, 50, 68),
            graph_sum_line_rgb: (205, 214, 244),
            graph_marker_rgb: vec![
                // amaranth pink
                (243, 167, 186),
                // cocktail red
                (253, 109, 114),
                // deep saffron
                (255, 150, 58),
                // corn
                (250, 234, 93),
                // mountain lake green
                (117, 185, 150),
                // cerulean
                (0, 143, 190),
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
                warn!(
                    "failed to load configuration from file, caused by: '{}'",
                    error
                );
                let config = Self::default();
                let Ok(downcast_error) = error.downcast::<std::io::Error>() else {
                    return config;
                };
                if downcast_error.kind() == std::io::ErrorKind::NotFound {
                    match config.to_file(filepath) {
                        Ok(()) => warn!(
                            "created default configuration file, at: '{}'",
                            filepath.display()
                        ),
                        Err(error) => warn!(
                            "failed to create default configuration file, at: '{}', caused by: '{}'",
                            filepath.display(),
                            error
                        ),
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
