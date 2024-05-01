use anyhow::Context;
use crossterm::style::Stylize;
use crate::warn;

use super::syrtime::{Blocs, SyrDate};
use std::{fs, io::{Read, Write}, path::Path};


pub struct Entries(Vec<Entry>);

impl From<Vec<Entry>> for Entries {
    fn from(value: Vec<Entry>) -> Self {
        Self(value)
    }
}

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
    pub fn load() -> anyhow::Result<Self> {
        Ok(
        std::path::Path::read_dir(crate::dirs::Dirs::get().data_dir())?
            .flat_map(|res| {
                match res {
                    Ok(entry) => {
                        let path = entry.path();
                        if path.extension()?.to_str()? == "json" {
                            match Entry::from_file(&path) {
                                Ok(entry) => Some(entry),
                                Err(error) => {
                                    warn!("{}", error);
                                    None
                                }
                            }
                        } else {
                            None
                        }
                    },
                    Err(error) => {
                        warn!("{}", error);
                        None
                    }
                }
            })
            .collect::<Vec<Entry>>().into()
        )
    }
}


pub struct Entry {
    name: String,
    aliases: Vec<String>,
    blocs: Blocs,
}

impl Entry {
    pub fn new(name: String, aliases: Vec<String>) -> Self {
        Self { name, aliases, blocs: Blocs::default() }
    }

    fn from_file(filepath: &Path) -> anyhow::Result<Self> {
        let file_name = filepath.file_stem().with_context(|| format!("failed to obtain filestem of : {}", filepath.display()))?
            .to_str().with_context(|| format!("filename OsStr cannot be converted to valid utf-8 : {}", filepath.display()))?;
        let (name, aliases) : (String, Vec<String>) = {
            if let Some((name, aliases)) = file_name.split_once('_') {
                (name.to_string(), aliases.split('_').map(|s| s.to_string()).collect())
            } else {
                (file_name.to_string(), Vec::new())
            }
        };

        let mut buffer: Vec<u8> = Vec::new();
        std::fs::OpenOptions::new()
            .create(false)
            .read(true)
            .open(filepath)?
            .read_to_end(&mut buffer)?;

        Ok(Self { name, aliases, blocs: serde_json::from_slice(&buffer)? })   
    }

    pub fn to_file(&self) -> anyhow::Result<()> {
        let mut filename = self.name.clone();
        filename.push_str("-·-");
        filename.push_str(&self.aliases.join("-·-"));
        filename.push_str(".json");

        let filepath = crate::dirs::Dirs::get().data_dir().join(filename);
        dbg!(filepath.clone());
        let data = serde_json::to_vec_pretty(&self.blocs)?;

        fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&filepath)?
            .write_all(&data)?;

        Ok(())
    }

    fn increase_bloc_duration(&mut self, date: &SyrDate, duration: u32) {
        if let Some(val) = self.blocs.get_mut(date) {
            *val += duration
        }
    }

    fn decrease_bloc_duration(&mut self, date: &SyrDate, duration: u32) {
        if let Some(val) = self.blocs.get_mut(date) {
            if duration > *val {
                *val = 0
            } else {
                *val -= duration
            }
        }
    }
}
