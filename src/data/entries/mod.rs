// Modules
mod ientries;
mod uentries;

// Re-exports
pub use ientries::IEntries;
pub use uentries::UEntries;

use super::Entry;
use std::slice::{Iter, IterMut};

#[derive(Debug)]
pub struct Entries<const I: bool>(pub Vec<Entry<I>>);

impl<const I: bool> Entries<I> {
    pub fn new_empty() -> Self {
        Self(Vec::new())
    }
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

impl<const I: bool> AsRef<[Entry<I>]> for Entries<I> {
    fn as_ref(&self) -> &[Entry<I>] {
        &self.0
    }
}
