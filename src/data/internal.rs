use std::io::{Read, Write};
use std::ops::AddAssign;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Entries(Vec<Entry>);

impl std::ops::Deref for Entries {
    type Target = Vec<Entry>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Entries {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Entries {
    const MAIN_FILEPATH: &'static str = "./syracuse.bin";
    const BACKUPS_PATH: &'static str = "./backups/";

    pub fn load() -> anyhow::Result<Self> {
        let mut buffer: Vec<u8> = Vec::new();
        std::fs::OpenOptions::new()
            .read(true)
            .open(Self::MAIN_FILEPATH)?
            .read_to_end(&mut buffer)?;
        Ok(bincode::deserialize(&buffer)?)
    }

    pub fn save(&self) -> anyhow::Result<()> {
        Ok(std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(Self::MAIN_FILEPATH)?
            .write_all(&bincode::serialize(&self)?)?)
    }

    pub fn backup(&self) -> anyhow::Result<()> {
        let timestamp = time::OffsetDateTime::now_utc().unix_timestamp();
        let filepath: String = format!("{}syracuse-backup-{}.bin", Self::BACKUPS_PATH, timestamp);
        Ok(std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&filepath)?
            .write_all(&bincode::serialize(&self)?)?)
    }

}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    names: Vec<String>,
    blocs: Vec<Bloc>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bloc {
    date: time::Date,
    duration: time::Duration,
}

impl Bloc {
    fn update_duration(&mut self, other: time::Duration) {
        self.duration.add_assign(other)
    }
}