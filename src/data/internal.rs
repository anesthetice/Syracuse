use crate::{
    algorithms,
    utils::{enter_clean_input_mode, exit_clean_input_mode, print_arrow},
};
use anyhow::Context;
use crossterm::{event, style::Stylize};
use itertools::Itertools;
use std::{
    io::{Read, Write},
    path::{Path, PathBuf},
};

use super::syrtime::{blocs::Blocs, syrdate::SyrDate};

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

pub enum IndexOptions {
    All,
    Indexed,
    Unindexed,
}

impl Entries {
    pub fn as_inner(&self) -> Vec<&Entry> {
        self.iter().collect_vec()
    }
    pub fn load() -> anyhow::Result<Self> {
        log::debug!("Loading entries...");
        let entries = Path::read_dir(crate::dirs::Dirs::get().data_dir())?
            .flat_map(|res| {
                let path = match res {
                    Ok(e) => e,
                    Err(err) => {
                        log::warn!("{}", err);
                        return None;
                    }
                }
                .path();
                if path.extension()?.to_str()? != "json" {
                    return None;
                }
                match Entry::from_file(&path) {
                    Ok(entry) => Some(entry),
                    Err(error) => {
                        log::warn!("{}", error);
                        None
                    }
                }
            })
            .collect::<Vec<Entry>>()
            .into();

        Ok(entries)
    }
    pub fn choose(&self, query: &str, index_options: IndexOptions) -> Option<Entry> {
        let choices: Vec<&Entry> = self
            .iter()
            // keeps only entries marked as indexed if indexed_exclusive is true
            .filter(|entry| match index_options {
                IndexOptions::All => true,
                IndexOptions::Indexed => entry.indexed,
                IndexOptions::Unindexed => !entry.indexed,
            })
            // outputs (score, entry)
            .map(|entry| {
                (
                    entry
                        .aliases
                        .iter()
                        .chain(std::iter::once(&entry.name))
                        .map(|string| {
                            let sw_factor = crate::config::Config::get().sw_nw_ratio;
                            sw_factor * algorithms::smith_waterman(string, query)
                                + (1.0 - sw_factor) * algorithms::needleman_wunsch(string, query)
                        })
                        .fold(-1.0, |acc, x| if x > acc { x } else { acc }),
                    entry,
                )
            })
            // keeps entries with a high enough score
            .filter(|(score, entry)| {
                log::trace!("{:<15}:   {:.3}", entry.name, score);
                *score >= crate::config::Config::get().search_threshold
            })
            // sorts by the entry with the highest score
            .sorted_by(|(a, _), (b, _)| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal))
            // keeps only the top 3
            .take(3)
            .map(|(_, entry)| entry)
            .collect();

        let response = match choices.len() {
            0 => None,
            1 => Self::choose_single(choices[0]),
            2.. => Self::choose_multiple(&choices),
        };

        print_arrow(
            match response.as_ref() {
                Some(entry) => &entry.name,
                None => "None",
            },
            "cyan",
        );
        response
    }
    fn choose_single(choice: &Entry) -> Option<Entry> {
        println!("{} [Y/n]", choice);
        enter_clean_input_mode();
        loop {
            if !event::poll(std::time::Duration::from_millis(200)).unwrap_or_else(|err| {
                log::warn!("Event polling issue: '{}'", err);
                false
            }) {
                continue;
            }
            let key = match event::read() {
                Ok(event::Event::Key(key)) => key,
                Ok(_) => continue,
                Err(error) => {
                    log::warn!("Event read issue: '{}'", error);
                    continue;
                }
            };

            if key.kind != event::KeyEventKind::Press {
                continue;
            }

            match key.code {
                event::KeyCode::Esc
                | event::KeyCode::Char('Q')
                | event::KeyCode::Char('q')
                | event::KeyCode::Char('N')
                | event::KeyCode::Char('n') => {
                    exit_clean_input_mode();
                    break None;
                }
                event::KeyCode::Char('y') | event::KeyCode::Enter => {
                    exit_clean_input_mode();
                    break Some(choice.clone());
                }
                _ => (),
            }
        }
    }
    fn choose_multiple(choices: &[&Entry]) -> Option<Entry> {
        for (idx, choice) in choices.iter().enumerate() {
            println!("{}. {}", idx + 1, choice);
        }
        enter_clean_input_mode();
        loop {
            if !event::poll(std::time::Duration::from_millis(200)).unwrap_or_else(|err| {
                log::warn!("Event polling issue: '{}'", err);
                false
            }) {
                continue;
            }
            let key = match event::read() {
                Ok(event::Event::Key(key)) => key,
                Ok(_) => continue,
                Err(error) => {
                    log::warn!("Event read issue: '{}'", error);
                    continue;
                }
            };

            if key.kind != event::KeyEventKind::Press {
                continue;
            }

            match key.code {
                event::KeyCode::Esc
                | event::KeyCode::Char('Q')
                | event::KeyCode::Char('q')
                | event::KeyCode::Char('N')
                | event::KeyCode::Char('n') => {
                    exit_clean_input_mode();
                    break None;
                }
                event::KeyCode::Enter => {
                    exit_clean_input_mode();
                    break Some(choices[0].clone());
                }
                event::KeyCode::Char(chr) => {
                    if !chr.is_numeric() {
                        continue;
                    }
                    let Ok(idx) = chr.to_string().parse::<usize>() else {
                        continue;
                    };
                    if let Some(entry) = choices.get(idx - 1) {
                        exit_clean_input_mode();
                        break Some((*entry).clone());
                    }
                }
                _ => {}
            }
        }
    }
    // path must be validated beforehand
    pub fn backup(&self, path: PathBuf) {
        log::debug!(
            "Attempting to back up entries to: '{}' ...",
            &path.display()
        );
        for entry in self.iter() {
            if let Err(error) = entry.save_to_file(&path.join(entry.get_filestem() + ".json")) {
                log::warn!("Failed to back up an entry: '{error}'")
            }
        }
    }
}

#[derive(Clone)]
pub struct Entry {
    pub(super) name: String,
    pub(super) aliases: Vec<String>,
    pub(super) blocs: Blocs,
    pub(super) indexed: bool,
}

impl std::fmt::Debug for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}; {}\n―――――――――――――――\n{}",
            self.name,
            self.aliases.join(", ").dim(),
            self.blocs
        )
    }
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}; {}", self.name, self.aliases.join(", ").dim())
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Entry {
    pub fn new(name: String, aliases: Vec<String>) -> Self {
        Self {
            name,
            aliases,
            blocs: Blocs::default(),
            indexed: true,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    fn from_file(filepath: &Path) -> anyhow::Result<Self> {
        log::debug!("Loading entry from '{}' filepath...", filepath.display());
        let separator: &str = crate::config::Config::get()
            .entry_file_name_separtor
            .as_str();
        let mut file_name = filepath
            .file_stem()
            .context("Failed to obtain filestem of: '{}'")?
            .to_str()
            .context("Filename OsStr cannot be converted to valid utf-8")?;

        let indexed = !file_name.ends_with(".noindex");
        if !indexed {
            file_name = &file_name[..(file_name.len() - 8)];
        }

        let (name, aliases): (String, Vec<String>) = {
            if let Some((name, aliases)) = file_name.split_once(separator) {
                (
                    name.to_string(),
                    aliases.split(separator).map(|s| s.to_string()).collect(),
                )
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

        Ok(Self {
            name,
            aliases,
            blocs: ijson::from_value(&serde_json::from_slice(&buffer)?)?,
            indexed,
        })
    }

    pub fn get_filestem(&self) -> String {
        let separator: &str = crate::config::Config::get()
            .entry_file_name_separtor
            .as_str();
        let mut filestem = self.name.clone();
        if !self.aliases.is_empty() {
            filestem.push_str(separator)
        }
        filestem.push_str(&self.aliases.join(separator));
        filestem
    }

    fn get_filepath(&self) -> PathBuf {
        crate::dirs::Dirs::get().data_dir().join(
            self.get_filestem()
                + if self.indexed {
                    ".json"
                } else {
                    ".noindex.json"
                },
        )
    }

    // true = valid, false = invalid
    pub fn is_new_entry_name_valid(&self, new_entry_name: &str) -> bool {
        !(self.name.as_str() == new_entry_name
            || self.aliases.iter().any(|alias| alias == new_entry_name))
    }

    pub fn get_bloc_duration(&self, date: &SyrDate) -> f64 {
        *self.blocs.get(date).unwrap_or(&0.0)
    }

    pub fn get_bloc_duration_total_as_hours(&self) -> f64 {
        self.blocs
            .iter()
            .flat_map(|(_, x)| if *x != 0.0 { Some(*x) } else { None })
            .fold(0_f64, |acc, x| acc + x / 3600.0)
    }

    pub fn save(&self) -> anyhow::Result<()> {
        self.save_to_file(&self.get_filepath())
    }

    pub(super) fn save_to_file(&self, filepath: &Path) -> anyhow::Result<()> {
        let parent_path = filepath.parent().unwrap_or(filepath).display();
        log::debug!("Attempting to save '{}' to '{}'...", self.name, parent_path);
        let data = serde_json::to_vec_pretty(&ijson::to_value(&self.blocs)?)?;

        std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(filepath)?
            .write_all(&data)?;

        log::info!("Saved '{}' to '{}'", self.name, parent_path);
        Ok(())
    }

    pub fn delete(self) -> anyhow::Result<()> {
        let filepath = self.get_filepath();
        log::debug!(
            "Attempting to remove '{}' from '{}'...",
            self.name,
            filepath.display()
        );
        std::fs::remove_file(filepath)?;
        log::info!("Removed '{}'", self.name);
        Ok(())
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

    pub fn prune(&mut self, cutoff_date: &SyrDate) -> anyhow::Result<usize> {
        let num = self.blocs.prune(cutoff_date);
        self.save()?;
        Ok(num)
    }

    pub fn is_indexed(&self) -> bool {
        self.indexed
    }

    pub fn inverse_indexability(&mut self) -> anyhow::Result<()> {
        let old_filepath = self.get_filepath();
        self.indexed = !self.indexed;
        std::fs::rename(old_filepath, self.get_filepath())?;
        Ok(())
    }
}

#[cfg(feature = "twotothree")]
pub fn convert(mut entries: Entries) -> anyhow::Result<()> {
    for entry in entries.0.iter_mut() {
        entry.blocs.iter_mut().for_each(|(_, val)| {
            if *val > 43200.0 {
                *val /= 1e9
            }
        });
        entry.save()?;
    }
    Ok(())
}
