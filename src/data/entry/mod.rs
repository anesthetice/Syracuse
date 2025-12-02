// Modules
mod anyentry;
mod display;
mod ientry;
mod uentry;

// Re-exports
pub use anyentry::AnyEntry;
pub use ientry::IEntry;
pub use uentry::UEntry;

use crate::data::entry::display::DisplayEntry;

use super::syrtime::{Blocks, SyrDate};
use crossterm::style::Stylize;

#[derive(Clone)]
pub struct Entry<const I: bool> {
    pub name: String,
    pub aliases: Vec<String>,
    pub blocks: Blocks,
}

pub trait EntryCore: Clone {
    fn get_name(&self) -> &str;
    fn get_aliases(&self) -> &[String];
    fn is_new_entry_name_valid(&self, new_entry_name: &str) -> bool;
    fn get_bloc_duration(&self, date: &SyrDate) -> f64;
    fn get_block_duration_opt(&self, date: &SyrDate) -> Option<f64>;
    fn display(&self) -> DisplayEntry;
}

impl<const I: bool> EntryCore for Entry<I> {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_aliases(&self) -> &[String] {
        &self.aliases
    }

    fn is_new_entry_name_valid(&self, new_entry_name: &str) -> bool {
        self.aliases
            .iter()
            .chain(std::iter::once(&self.name))
            .any(|name| name == new_entry_name)
    }

    fn get_bloc_duration(&self, date: &SyrDate) -> f64 {
        *self.blocks.get(date).unwrap_or(&0.0)
    }

    fn get_block_duration_opt(&self, date: &SyrDate) -> Option<f64> {
        self.blocks.get(date).cloned()
    }

    fn display(&self) -> DisplayEntry {
        self.into()
    }
}

impl<const I: bool> Entry<I> {
    pub fn new(name: String, aliases: Vec<String>, blocks: Blocks) -> Self {
        Self {
            name,
            aliases,
            blocks,
        }
    }

    pub fn create(name: String, aliases: Vec<String>) -> Self {
        Self::new(name, aliases, Blocks::default())
    }
}

impl<const I: bool> PartialEq for Entry<I> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
