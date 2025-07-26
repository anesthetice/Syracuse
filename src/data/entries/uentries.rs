use std::{io::Read, path::Path};

use color_eyre::eyre;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::data::UEntry;

use super::Entries;

pub type UEntries = Entries<false>;

impl UEntries {
    pub fn load(filepath: &Path) -> eyre::Result<Self> {
        let mut buffer: Vec<u8> = Vec::new();
        std::fs::OpenOptions::new()
            .create(false)
            .read(true)
            .open(filepath)?
            .read_to_end(&mut buffer)?;

        Ok(Self(ijson::from_value(&serde_json::from_slice(&buffer)?)?))
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
