use crossterm::style::Stylize;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

use crate::{data::internal::UniColor, warn};

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub search_threshold: f64,
    pub local_offset: [i8; 3],
    pub backup_expiration_time: u64,
    pub save_period: f64,
    pub colorful: bool,
    pub color_green: UniColor,
    pub color_red: UniColor,
    pub color_palette: Vec<UniColor>,
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
            color_green: UniColor::new(0, 128, 0),
            color_red: UniColor::new(128, 0, 0),
            color_palette: vec![
                UniColor::new(242, 213, 207),
                UniColor::new(238, 190, 190),
                UniColor::new(244, 184, 228),
                UniColor::new(202, 158, 230),
                UniColor::new(231, 130, 132),
                UniColor::new(234, 153, 156),
                UniColor::new(239, 159, 118),
                UniColor::new(229, 200, 144),
                UniColor::new(166, 209, 137),
                UniColor::new(129, 200, 190),
                UniColor::new(153, 209, 219),
                UniColor::new(133, 193, 220),
                UniColor::new(140, 170, 238),
                UniColor::new(186, 187, 241),
            ],
            graph_specific_end_date: None,
            graph_num_of_days_back: 13,
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
