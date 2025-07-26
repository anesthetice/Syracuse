// Modules
mod ientries;
mod uentries;

// Re-exports
pub use ientries::IEntries;
pub use uentries::UEntries;

use std::slice::{Iter, IterMut};

use crate::{
    algorithms,
    utils::{ARROW, enter_clean_input_mode, exit_clean_input_mode},
};
use color_eyre::Result;
use crossterm::{event, style::Stylize};
use itertools::Itertools;

use super::{Entry, EntryCore, IEntry, IndexOptions, UEntry};

pub struct Entries<const I: bool>(Vec<Entry<I>>);

impl<const I: bool> Entries<I> {
    pub fn iter(&self) -> Iter<'_, Entry<I>> {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, Entry<I>> {
        self.0.iter_mut()
    }
    pub fn into_iter(self) -> std::vec::IntoIter<Entry<I>> {
        self.0.into_iter()
    }
}

impl<const I: bool> From<Vec<Entry<I>>> for Entries<I> {
    fn from(value: Vec<Entry<I>>) -> Self {
        Self(value)
    }
}

/*
impl Entries {
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

*/
