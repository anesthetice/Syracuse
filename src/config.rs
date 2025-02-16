use crate::{animation::AnimationBuilder, data::graphing::interpolation::InterpolationMethod};
use color_eyre::Result;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    sync::OnceLock,
};

pub static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Determines what set of characters separate the names of an entry stored as a file.
    pub entry_file_name_separtor: String,
    /// Determines how often should progress be automatically saved in seconds.
    pub autosave_period: u16,
    // The default backup path.
    pub backup_path: String,
    /// Determines weather or not the initial time is displayed when running an entry.
    pub stopwatch_explicit: bool,
    /// Determines the numbers of hours past midnight for which running a command will count for the previous day.
    pub night_owl_hour_extension: i8,

    /// The threshold for results to be considered.
    pub search_threshold: f64,
    /// The relative weights of the Smith-Waterman and Needlman-Wunsch algorithms respectively.
    pub sw_nw_ratio: f64,
    /// The match score used by both algorithms.
    pub match_score: i16,
    /// The mismatch penalty used by both algorithms.
    pub mismatch_penalty: i16,
    /// The gap penalty used by both algorithms.
    pub gap_penalty: i16,

    /// Determines how long in milliseconds a frame will be displayed before being refreshed.
    pub frame_period: u64,
    /// The animation frames, an array containing (left, right) strings.
    pub animation: AnimationBuilder,

    /// Determines the directory where graphs are saved, an empty string defaults to current directory.
    pub graph_output_dir: String,
    /// Determines the interpolation method used, "Linear" and "Makima" are currently available.
    pub graph_interpolation_method: InterpolationMethod,
    /// Determines the number of points between a date and the next one that will be interpolated.
    pub graph_nb_interpolated_points: usize,
    /// Determines the marker size for entries.
    pub graph_marker_size: u32,
    /// Determines the background color of the graph.
    pub graph_background_rgb: (u8, u8, u8),
    /// Determines the foreground color of the graph.
    pub graph_foreground_rgb: (u8, u8, u8),
    /// Determines the bold grid color of the graph.
    pub graph_coarse_grid_rgb: (u8, u8, u8),
    /// Determines the fine grid color of the graph.
    pub graph_fine_grid_rgb: (u8, u8, u8),
    /// Determines the sum line color of the graph.
    pub graph_sum_line_rgb: (u8, u8, u8),
    /// Determines the colors used for entry markers.
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
            Err(err) => {
                eprintln!("Warning: Failed to load configuration from file, '{}'", err);
                let config = Self::default();
                let Ok(downcast_error) = err.downcast::<std::io::Error>() else {
                    return config;
                };
                if downcast_error.kind() == std::io::ErrorKind::NotFound {
                    match config.to_file(filepath) {
                        Ok(()) => eprintln!("Warning: Created default configuration file, at '{}'", filepath.display()),
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
        std::fs::OpenOptions::new().create(false).read(true).open(filepath)?.read_to_end(&mut buffer)?;
        Ok(ijson::from_value(&serde_json::from_slice(&buffer)?)?)
    }

    fn to_file(&self, filepath: &std::path::Path) -> Result<()> {
        let mut file = std::fs::OpenOptions::new().write(true).create_new(true).open(filepath)?;

        file.write_all(&serde_json::to_vec_pretty(&ijson::to_value(self)?)?)?;
        file.flush()?;
        Ok(())
    }
}
