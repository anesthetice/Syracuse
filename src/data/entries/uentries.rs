use std::{
    io::{Read, Write},
    path::Path,
};

use color_eyre::eyre;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::data::UEntry;

use super::Entries;

pub type UEntries = Entries<false>;

impl UEntries {
    /// In all Lowercase so not an issue for now, might need guardrails
    /// if in the future if we decide to drop strict entry uppercasing.
    pub const FILENAME: &str = "_unindexed.json";

    pub fn load_from_default_file(data_dir: &Path) -> eyre::Result<Self> {
        Self::load_from_file(&data_dir.join(Self::FILENAME))
    }

    fn load_from_file(filepath: &Path) -> eyre::Result<Self> {
        let mut buffer: Vec<u8> = Vec::new();
        std::fs::OpenOptions::new()
            .create(false)
            .read(true)
            .open(filepath)?
            .read_to_end(&mut buffer)?;

        Ok(Self(serde_json::from_slice(&buffer)?))
    }

    pub fn save_to_default_file(&self, data_dir: &Path) -> eyre::Result<()> {
        self.save_to_file(&data_dir.join(Self::FILENAME))
    }

    pub fn save_to_file(&self, filepath: &Path) -> eyre::Result<()> {
        let data = serde_json::to_vec_pretty(&self)?;

        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(filepath)?;

        file.write_all(&data)?;
        file.sync_all()?;

        Ok(())
    }
}

impl Serialize for UEntries {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for UEntries {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self(<Vec<UEntry>>::deserialize(deserializer)?))
    }
}
