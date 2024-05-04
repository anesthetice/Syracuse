use anyhow::Context;
use crossterm::{event, style::Stylize};
use itertools::Itertools;
use crate::{algorithms, info, utils::{enter_clean_input_mode, exit_clean_input_mode}, warn};

use super::syrtime::{Blocs, SyrDate};
use std::{io::{Read, Write}, path::Path};


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
    pub fn choose(&self, query: &str) -> Option<Entry> {
        let choices: Vec<&Entry> = self.iter()
            .map(|entry| {
                (entry.aliases
                    .iter()
                    .chain(std::iter::once(&entry.name))
                    .map(|string| {
                        let sw_factor = crate::config::Config::get().sw_nw_ratio;
                        sw_factor * algorithms::smith_waterman(string, query)
                        + (1.0-sw_factor) * algorithms::needleman_wunsch(string, query)
                    })
                    .fold(-1.0, |acc, x| {if x > acc {x} else {acc}}),
                entry)
            })
            .filter(|(score, entry)| {
                info!("{:<15}:   {:.3}", entry.name, score);
                *score >= crate::config::Config::get().search_threshold
            })
            .sorted_by(|(a, _), (b, _)| {b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal)})
            .take(3)
            .map(|(_, entry)| {entry})
            .collect();
        
        match choices.len() {
            0 => None,
            1 => Self::choose_single(choices[0]),
            2.. => Self::choose_multiple(&choices)
        }
    }
    fn choose_single(choice: &Entry) -> Option<Entry> {
        println!("{} [Y/n]", choice);
        enter_clean_input_mode();
        loop {
            if !event::poll(std::time::Duration::from_millis(200)).unwrap_or_else(|err| {
                warn!("event polling issue, {}", err);
                false
            })
            {
                continue;
            }
            let key = match event::read() {
                Ok(event::Event::Key(key)) => key,
                Ok(_) => continue,
                Err(error) => {
                    warn!("event read issue, {}", error);
                    continue
                },
            };

            if key.kind != event::KeyEventKind::Press {
                continue;
            }
            match key.code {
                event::KeyCode::Esc
                | event::KeyCode::Char('q')
                | event::KeyCode::Char('n') => {
                    exit_clean_input_mode();
                    break None;
                }
                event::KeyCode::Char('y') | event::KeyCode::Enter => {
                    exit_clean_input_mode();
                    break Some(choice.clone());
                }
                _ => {}
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
                warn!("event polling issue, {}", err);
                false
            })
            {
                continue;
            }
            let key = match event::read() {
                Ok(event::Event::Key(key)) => key,
                Ok(_) => continue,
                Err(error) => {
                    warn!("event read issue, {}", error);
                    continue
                },
            };

            if key.kind != event::KeyEventKind::Press {
                continue;
            }
            match key.code {
                event::KeyCode::Esc
                | event::KeyCode::Char('q')
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
                    if let Some(entry) = choices.get(idx) {
                        exit_clean_input_mode();
                        break Some((*entry).clone());
                    }
                }
                _ => {}
            }

        }
    }
}

#[derive(Clone)]
pub struct Entry {
    name: String,
    aliases: Vec<String>,
    blocs: Blocs,
}

impl std::fmt::Debug for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}; {}\n―――――――――――――――\n{}",
            self.name,
            self.aliases.join(", "),
            self.blocs
        )
    }
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}; {}",
            self.name,
            self.aliases.join(", ")
        )
    }
}

impl Entry {
    pub fn new(name: String, aliases: Vec<String>) -> Self {
        Self { name, aliases, blocs: Blocs::default() }
    }

    fn from_file(filepath: &Path) -> anyhow::Result<Self> {
        let separator: &str = crate::config::Config::get().entry_file_name_separtor.as_str();
        let file_name = filepath.file_stem().with_context(|| format!("failed to obtain filestem of : {}", filepath.display()))?
            .to_str().with_context(|| format!("filename OsStr cannot be converted to valid utf-8 : {}", filepath.display()))?;
        let (name, aliases) : (String, Vec<String>) = {
            if let Some((name, aliases)) = file_name.split_once(separator) {
                (name.to_string(), aliases.split(separator).map(|s| s.to_string()).collect())
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

    pub fn get_filestem(&self) -> String {
        let separator: &str = crate::config::Config::get().entry_file_name_separtor.as_str();
        let mut filestem = self.name.clone();
        filestem.push_str(separator);
        filestem.push_str(&self.aliases.join(separator));
        filestem
    }

    fn get_filepath(&self) -> std::path::PathBuf {
        crate::dirs::Dirs::get()
            .data_dir()
            .join(self.get_filestem() + ".json")
    }

    pub fn save_to_file(&self) -> anyhow::Result<()> {
        let data = serde_json::to_vec_pretty(&self.blocs)?;

        std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(self.get_filepath())?
            .write_all(&data)?;

        Ok(())
    }

    pub fn delete(self) -> anyhow::Result<()> {
        Ok(std::fs::remove_file(self.get_filepath())?)
    }

    pub fn increase_bloc_duration(&mut self, date: &SyrDate, duration: u128) {
        if let Some(val) = self.blocs.get_mut(date) {
            *val += duration
        } else {
            self.blocs.insert(*date, duration);
        }
    }

    pub fn decrease_bloc_duration(&mut self, date: &SyrDate, duration: u128) {
        let mut delete_bloc: bool = false;
        if let Some(val) = self.blocs.get_mut(date) {
            if duration > *val {
                delete_bloc = true;
            } else {
                *val -= duration
            }
        }
        if delete_bloc {
            if self.blocs.remove(date).is_none() {
                warn!("failed to decrease duration, could not remove bloc")
            }
        }
    }
}
