use crate::{info, warn};
use crossterm::style::{Color, Stylize};
use serde::{Serialize, Deserialize};
use std::io::{Read, Write};

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub search_threshold: f64,
    pub local_offset: [i8; 3],
    pub backup_expiration_time: u64,
    pub save_period: f64,
    pub colorful: bool,
    pub color_green: Color,
    pub color_red: Color,
    pub color_palette: Vec<Color>,
    pub graph_specific_end_date: Option<time::Date>,
    pub graph_num_of_days_back: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            search_threshold: 0.8,
            local_offset: [0, 0, 0],
            backup_expiration_time: 172800,
            save_period: 15.0,
            colorful: true,
            color_green: Color::Rgb { r: 166, g: 209, b: 137 },
            color_red: Color::Rgb { r: 231, g: 130, b: 132 },
            color_palette: vec![
                Color::Rgb { r: 238, g: 190, b: 190 },
                Color::Rgb { r: 202, g: 158, b: 230 },
                Color::Rgb { r: 234, g: 153, b: 156 },
                Color::Rgb { r: 229, g: 200, b: 144 },
                Color::Rgb { r: 129, g: 200, b: 190 },
                Color::Rgb { r: 133, g: 193, b: 220 },
                Color::Rgb { r: 186, g: 187, b: 241 },
            ],
            graph_specific_end_date: None,
            graph_num_of_days_back: 14,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        match Self::load() {
            Ok(config) => config,
            Err(error) => {
                warn!("failed to load configuration\n{}", error);
                let config = Self::default();
                if let Err(error) = config.save() {
                    warn!("failed to save generated config\n{}", error);
                }
                config
            }
        }
    }

    fn save(&self) -> anyhow::Result<()> {
        let serialized_data: Vec<u8> = serde_json::to_vec_pretty(&self)?;

        std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("syracuse.config")?
            .write_all(&serialized_data)?;

        Ok(())
    }

    fn load() -> anyhow::Result<Self> {
        let mut buffer: Vec<u8> = Vec::with_capacity(1024);
        std::fs::OpenOptions::new()
            .create(false)
            .read(true)
            .open("syracuse.config")?
            .read_to_end(&mut buffer)?;

        Ok(serde_json::from_slice(&buffer)?)
    }
}