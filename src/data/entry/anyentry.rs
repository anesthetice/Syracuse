use itertools::Itertools;

use super::Entry;
use super::EntryCore;
use crate::data::{Blocs, SyrDate};

#[derive(Clone)]
pub struct AnyEntry<'a> {
    pub name: &'a str,
    pub aliases: &'a [String],
    pub blocs: &'a Blocs,
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
        *self.blocs.get(date).unwrap_or(&0.0)
    }

    fn get_block_duration_opt(&self, date: &SyrDate) -> Option<f64> {
        self.blocs.get(date).cloned()
    }

    fn print_name_and_first_alias(&self) -> String {
        match self.aliases.first() {
            Some(alias) => format!("{}; {}", self.name, alias),
            None => self.name.to_string(),
        }
    }
}

impl<'a, const I: bool> From<&'a Entry<I>> for AnyEntry<'a> {
    fn from(value: &'a Entry<I>) -> Self {
        Self {
            name: value.name.as_str(),
            aliases: &value.aliases,
            blocs: &value.blocs,
        }
    }
}
