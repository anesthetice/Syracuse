use std::collections::BTreeMap;

use crossterm::style::Stylize;

use crate::data::{AnyEntry, Blocks, Entry};

pub struct DisplayEntry<'a> {
    name: &'a str,
    aliases: &'a [String],
    blocks: &'a Blocks,
    params: DisplayParams,
}

pub struct DisplayParams {
    show_first_alias_only: bool,
    dim_aliases: bool,
    show_blocks: bool,
}

impl Default for DisplayParams {
    fn default() -> Self {
        Self {
            show_first_alias_only: true,
            dim_aliases: true,
            show_blocks: false,
        }
    }
}

impl<'a> std::fmt::Display for DisplayEntry<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params = &self.params;

        if self.aliases.is_empty() {
            write!(f, "{}", self.name)?
        } else {
            let aliases = if params.show_first_alias_only {
                self.aliases[0].clone()
            } else {
                self.aliases.join(", ")
            };
            if params.dim_aliases {
                write!(f, "{}; {}", self.name, aliases.dim())?;
            } else {
                write!(f, "{}; {}", self.name, aliases)?;
            }
        }

        if params.show_blocks {
            writeln!(f, "\n{}", self.blocks)?;
        }

        Ok(())
    }
}

impl<'a, const I: bool> From<&'a Entry<I>> for DisplayEntry<'a> {
    fn from(value: &'a Entry<I>) -> Self {
        Self {
            name: value.name.as_str(),
            aliases: &value.aliases,
            blocks: &value.blocks,
            params: Default::default(),
        }
    }
}

impl<'a> From<&AnyEntry<'a>> for DisplayEntry<'a> {
    fn from(value: &AnyEntry<'a>) -> Self {
        Self {
            name: value.name,
            aliases: value.aliases,
            blocks: value.blocks,
            params: Default::default(),
        }
    }
}

impl<'a> DisplayEntry<'a> {
    pub fn all_aliases(mut self) -> Self {
        self.params.show_first_alias_only = false;
        self
    }

    pub fn no_dim(mut self) -> Self {
        self.params.dim_aliases = false;
        self
    }

    pub fn show_blocks(mut self) -> Self {
        self.params.show_blocks = true;
        self
    }
}
