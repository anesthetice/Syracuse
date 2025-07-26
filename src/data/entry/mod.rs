mod anyentry;
mod ientry;
mod uentry;

pub use anyentry::AnyEntry;
pub use ientry::IEntry;
pub use uentry::UEntry;

use super::syrtime::{Blocs, SyrDate};
use crossterm::style::Stylize;

#[derive(Clone)]
pub struct Entry<const I: bool> {
    pub name: String,
    pub aliases: Vec<String>,
    pub blocs: Blocs,
}

pub trait EntryCore {
    fn is_new_entry_name_valid(&self, new_entry_name: &str) -> bool;
    fn get_bloc_duration(&self, date: &SyrDate) -> f64;
    fn get_block_duration_opt(&self, date: &SyrDate) -> Option<f64>;
    fn print_name_and_first_alias(&self) -> String;
}

impl<const I: bool> EntryCore for Entry<I> {
    fn is_new_entry_name_valid(&self, new_entry_name: &str) -> bool {
        self.aliases
            .iter()
            .chain(std::iter::once(&self.name))
            .any(|name| name == new_entry_name)
    }

    fn get_bloc_duration(&self, date: &SyrDate) -> f64 {
        *self.blocs.get(date).unwrap_or(&0.0)
    }

    fn get_block_duration_opt(&self, date: &SyrDate) -> Option<f64> {
        self.blocs.get(date).cloned()
    }

    fn print_name_and_first_alias(&self) -> String {
        match self.aliases.first() {
            Some(alias) => format!("{}; {}", self.name, alias),
            None => self.name.clone(),
        }
    }
}

impl<const I: bool> Entry<I> {
    pub const SEPARATOR: &'static str = "⧿"; // Miny. Miscellaneous Mathematical Symbols-B, U+29FF
    pub const EXTENSION: &'static str = ".json";

    pub fn new(name: String, aliases: Vec<String>, blocs: Blocs) -> Self {
        Self { name, aliases, blocs }
    }

    pub fn create(name: String, aliases: Vec<String>) -> Self {
        Self::new(name, aliases, Blocs::default())
    }
}

impl<const I: bool> std::fmt::Debug for Entry<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.aliases.len() {
            0 => write!(f, "{}\n{}", self.name, self.blocs),
            1.. => write!(f, "{}; {}\n{}", self.name, self.aliases.join(", ").dim(), self.blocs),
        }
    }
}

impl<const I: bool> std::fmt::Display for Entry<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.aliases.len() {
            0 => write!(f, "{}", self.name),
            1.. => write!(f, "{}; {}", self.name, self.aliases.join(", ").dim()),
        }
    }
}

impl<const I: bool> PartialEq for Entry<I> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
