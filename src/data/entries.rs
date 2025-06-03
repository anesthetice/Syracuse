use std::{
    slice::{Iter, IterMut},
    vec::IntoIter,
};

use crate::{
    algorithms,
    utils::{ARROW, enter_clean_input_mode, exit_clean_input_mode},
};
use color_eyre::Result;
use crossterm::{event, style::Stylize};
use itertools::Itertools;

use super::{Entry, EntryCore, IEntry, IndexOptions, UEntry};

pub struct Entries {
    indexed: Vec<IEntry>,
    unindexed: Vec<UEntry>,
}

impl Entries {
    pub fn iter_i(&self) -> Iter<'_, IEntry> {
        self.indexed.iter()
    }
    pub fn iter_mut_i(&mut self) -> IterMut<'_, IEntry> {
        self.indexed.iter_mut()
    }
    pub fn iter_u(&self) -> Iter<'_, UEntry> {
        self.unindexed.iter()
    }
    pub fn iter_mut_u(&mut self) -> IterMut<'_, UEntry> {
        self.unindexed.iter_mut()
    }
    pub fn iter_all(&self) -> impl Iterator<Item = &'_ dyn EntryCore> {
        let iter_i = self.iter_i().map(|e| e as &dyn EntryCore);
        let iter_u = self.iter_u().map(|e| e as &dyn EntryCore);
        iter_i.chain(iter_u)
    }

    pub fn load() -> Result<Self> {
        let indexed_entries: Vec<IEntry> = std::fs::read_dir(crate::dirs::Dirs::get().data_dir())?
            .filter_map(|res| {
                let path = match res {
                    Ok(e) => e,
                    Err(err) => {
                        eprintln!("Warning: {}", err);
                        return None;
                    }
                }
                .path();
                if path.extension()?.to_str()? != "json" {
                    return None;
                }
                if path.file_stem()?.to_str()? == Entry::UNINDEXED_FILE_STEM {
                    return None;
                }
                match Entry::from_file(&path) {
                    Ok(entry) => Some(entry),
                    Err(error) => {
                        eprintln!("Warning: {}", error);
                        None
                    }
                }
            })
            .collect();
        unimplemented!()
    }
    pub fn choose(&self, query: &str, index_options: IndexOptions) -> Option<Entry> {
        let sw_nw_ratio = crate::config::Config::get().sw_nw_ratio;
        let choices: Vec<&Entry> = self
            .iter()
            // Keeps only entries marked as indexed if indexed_exclusive is true
            .filter(|entry| match index_options {
                IndexOptions::All => true,
                IndexOptions::Indexed => entry.indexed,
                IndexOptions::Unindexed => !entry.indexed,
            })
            .map(|entry| {
                (
                    entry
                        .aliases
                        .iter()
                        .chain(std::iter::once(&entry.name))
                        .map(|string| {
                            sw_nw_ratio * algorithms::smith_waterman(string, query)
                                + (1.0 - sw_nw_ratio) * algorithms::needleman_wunsch(string, query)
                        })
                        .fold(-1.0, |acc, x| if x > acc { x } else { acc }),
                    entry,
                )
            })
            // Keeps entries with a high enough score
            .filter(|(score, _)| *score >= crate::config::Config::get().search_threshold)
            // Sorts by score, highest at the top
            .sorted_by(|(a, _), (b, _)| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal))
            // Keep only the top three scores
            .take(3)
            .map(|(_, entry)| entry)
            .collect();

        let response = match choices.len() {
            0 => None,
            1 => Self::choose_single(choices[0]),
            2.. => Self::choose_multiple(&choices),
        };

        if let Some(entry) = response.as_ref() {
            println!("{} {}", ARROW.cyan().dim(), entry.name.as_str().dim())
        }
        println!();
        response
    }
    fn choose_single(choice: &Entry) -> Option<Entry> {
        println!("{} [y/N]", choice);
        enter_clean_input_mode();
        loop {
            if !event::poll(std::time::Duration::from_millis(200)).unwrap_or_else(|err| {
                eprintln!("Warning: Event polling issue, '{}'", err);
                false
            }) {
                continue;
            }
            let key = match event::read() {
                Ok(event::Event::Key(key)) => key,
                Ok(_) => continue,
                Err(err) => {
                    eprintln!("Warning: Event read issue, '{}'", err);
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
                eprintln!("Warning: Event polling issue: '{}'", err);
                false
            }) {
                continue;
            }
            let key = match event::read() {
                Ok(event::Event::Key(key)) => key,
                Ok(_) => continue,
                Err(err) => {
                    eprintln!("Warning: Event read issue, '{}'", err);
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
}
