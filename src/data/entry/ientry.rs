use crate::data::{Entry, EntryCore, SyrDate};
use color_eyre::{
    Result,
    eyre::{OptionExt, eyre},
};
use itertools::Itertools;
use std::{
    io::{Read, Write},
    path::{Path, PathBuf},
};

pub type IEntry = Entry<true>;

impl Entry<true> {
    pub fn from_file(filepath: &Path) -> Result<Self> {
        let mut file_name = filepath
            .file_stem()
            .ok_or_else(|| eyre!("Failed to obtain filestem of: '{}'", filepath.display()))?
            .to_str()
            .ok_or_eyre("Failed to convert entry filename OsStr into &str")?;

        let indexed = !file_name.ends_with(".noindex");
        if !indexed {
            file_name = &file_name[..(file_name.len() - 8)];
        }

        let (name, aliases): (String, Vec<String>) = match file_name.split_once(Self::SEPARATOR) {
            Some((name, aliases)) => (
                name.to_string(),
                aliases.split(Self::SEPARATOR).map(|s| s.to_string()).collect(),
            ),
            None => (file_name.to_string(), Vec::new()),
        };

        let mut buffer: Vec<u8> = Vec::new();
        std::fs::OpenOptions::new()
            .create(false)
            .read(true)
            .open(filepath)?
            .read_to_end(&mut buffer)?;

        Ok(Self::new(
            name,
            aliases,
            ijson::from_value(&serde_json::from_slice(&buffer)?)?,
        ))
    }

    pub fn get_dirname() -> &'static Path {
        crate::dirs::Dirs::get().data_dir()
    }

    pub fn get_filestem(&self) -> String {
        std::iter::once(self.name.as_str())
            .chain(self.aliases.iter().map(String::as_str))
            .join(Self::SEPARATOR)
    }

    pub fn get_filename(&self) -> String {
        std::iter::once(self.name.as_str())
            .chain(self.aliases.iter().map(String::as_str))
            .join(Self::SEPARATOR)
            + Self::EXTENSION
    }

    pub fn get_filepath(&self) -> PathBuf {
        Self::get_dirname().join(self.get_filename())
    }

    pub fn save(&self) -> Result<()> {
        self.save_to_file(&self.get_filepath())
    }

    pub fn save_to_file(&self, filepath: &Path) -> Result<()> {
        let data = serde_json::to_vec_pretty(&ijson::to_value(&self.blocs)?)?;

        std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(filepath)?
            .write_all(&data)?;

        Ok(())
    }

    pub fn delete(self) -> Result<()> {
        std::fs::remove_file(self.get_filepath()).map_err(Into::into)
    }

    pub fn increase_bloc_duration(&mut self, date: &SyrDate, duration: f64) {
        if let Some(val) = self.blocs.get_mut(date) {
            *val += duration
        } else {
            self.blocs.insert(*date, duration);
        }
    }

    pub fn decrease_bloc_duration(&mut self, date: &SyrDate, duration: f64) {
        let mut delete_bloc: bool = false;
        if let Some(val) = self.blocs.get_mut(date) {
            if duration > *val {
                delete_bloc = true;
            } else {
                *val -= duration
            }
        }
        if delete_bloc {
            self.blocs.remove(date);
        }
    }
}

impl From<Entry<false>> for Entry<true> {
    fn from(value: Entry<false>) -> Self {
        Self::new(value.name, value.aliases, value.blocs)
    }
}
