use super::syrtime::{Blocs, SyrDate};
use color_eyre::{
    Result,
    eyre::{OptionExt, eyre},
};
use crossterm::style::Stylize;
use itertools::Itertools;
use std::{
    io::{Read, Write},
    path::{Path, PathBuf},
};

#[derive(Clone)]
pub struct Entry {
    pub name: String,
    pub aliases: Vec<String>,
    pub blocs: Blocs,
    pub indexed: bool,
}

impl std::fmt::Debug for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.aliases.len() {
            0 => write!(f, "{}\n{}", self.name, self.blocs),
            1.. => write!(f, "{}; {}\n{}", self.name, self.aliases.join(", ").dim(), self.blocs),
        }
    }
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.aliases.len() {
            0 => write!(f, "{}", self.name),
            1.. => write!(f, "{}; {}", self.name, self.aliases.join(", ").dim()),
        }
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Entry {
    pub fn new(name: String, aliases: Vec<String>, blocs: Blocs, indexed: bool) -> Self {
        Self {
            name,
            aliases,
            blocs,
            indexed,
        }
    }

    pub fn create(name: String, aliases: Vec<String>) -> Self {
        Self::new(name, aliases, Blocs::default(), true)
    }

    pub(super) fn from_file(filepath: &Path) -> Result<Self> {
        let separator: &str = crate::config::Config::get().entry_file_name_separtor.as_str();
        let mut file_name = filepath
            .file_stem()
            .ok_or_else(|| eyre!("Failed to obtain filestem of: '{}'", filepath.display()))?
            .to_str()
            .ok_or_eyre("Failed to convert entry filename OsStr into &str")?;

        let indexed = !file_name.ends_with(".noindex");
        if !indexed {
            file_name = &file_name[..(file_name.len() - 8)];
        }

        let (name, aliases): (String, Vec<String>) = match file_name.split_once(separator) {
            Some((name, aliases)) => (name.to_string(), aliases.split(separator).map(|s| s.to_string()).collect()),
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
            indexed,
        ))
    }

    pub fn get_filestem(&self) -> String {
        std::iter::once(self.name.as_str())
            .chain(self.aliases.iter().map(String::as_str))
            .join(crate::config::Config::get().entry_file_name_separtor.as_str())
    }

    pub fn get_extension(&self) -> &'static str {
        match self.indexed {
            true => ".json",
            false => ".noindex.json",
        }
    }

    pub fn get_filename(&self) -> String {
        std::iter::once(self.name.as_str())
            .chain(self.aliases.iter().map(String::as_str))
            .join(crate::config::Config::get().entry_file_name_separtor.as_str())
            + self.get_extension()
    }

    pub fn get_filepath(&self) -> PathBuf {
        crate::dirs::Dirs::get().data_dir().join(self.get_filename())
    }

    pub fn is_new_entry_name_valid(&self, new_entry_name: &str) -> bool {
        self.aliases
            .iter()
            .chain(std::iter::once(&self.name))
            .any(|name| name == new_entry_name)
    }

    pub fn get_bloc_duration(&self, date: &SyrDate) -> f64 {
        *self.blocs.get(date).unwrap_or(&0.0)
    }

    pub fn get_block_duration_opt(&self, date: &SyrDate) -> Option<f64> {
        self.blocs.get(date).cloned()
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

    pub fn inverse_indexability(&mut self) -> Result<()> {
        let old_filepath = self.get_filepath();
        self.indexed = !self.indexed;
        std::fs::rename(old_filepath, self.get_filepath())?;
        Ok(())
    }

    pub fn display_name_and_first_alias(&self) -> String {
        match self.aliases.first() {
            Some(alias) => format!("{}; {}", self.name, alias.as_str().dim()),
            None => self.name.clone(),
        }
    }

    pub fn print_name_and_first_alias(&self) -> String {
        match self.aliases.first() {
            Some(alias) => format!("{}; {}", self.name, alias),
            None => self.name.clone(),
        }
    }
}
