use itertools::Itertools;

use super::Entry;
use super::EntryCore;
use crate::data::{Blocks, SyrDate};

#[derive(Clone)]
pub struct AnyEntry<'a> {
    pub name: &'a str,
    pub aliases: &'a [String],
    pub blocks: &'a Blocks,
    pub indexed: bool,
}

impl<'a> EntryCore for AnyEntry<'a> {
    fn get_name(&self) -> &str {
        self.name
    }

    fn get_aliases(&self) -> &[String] {
        self.aliases
    }

    fn is_new_entry_name_valid(&self, new_entry_name: &str) -> bool {
        self.aliases
            .iter()
            .map(String::as_str)
            .chain(std::iter::once(self.name))
            .any(|name| name == new_entry_name)
    }

    fn get_bloc_duration(&self, date: &SyrDate) -> f64 {
        *self.blocks.get(date).unwrap_or(&0.0)
    }

    fn get_block_duration_opt(&self, date: &SyrDate) -> Option<f64> {
        self.blocks.get(date).cloned()
    }

    fn display(&self) -> super::display::DisplayEntry {
        self.into()
    }
}

impl<'a, const I: bool> From<&'a Entry<I>> for AnyEntry<'a> {
    fn from(value: &'a Entry<I>) -> Self {
        Self {
            name: value.name.as_str(),
            aliases: &value.aliases,
            blocks: &value.blocks,
            indexed: I,
        }
    }
}
