use std::path::Path;

use color_eyre::eyre;
use itertools::Itertools;

use crate::data::IEntry;

use super::Entries;

pub type IEntries = Entries<true>;

impl IEntries {
    pub fn load(path: &Path) -> eyre::Result<Self> {
        Ok(std::fs::read_dir(path)?
            .filter_map(|res| {
                let path = match res {
                    Ok(e) => e.path(),
                    Err(err) => {
                        eprintln!("Warning: {}", err);
                        return None;
                    }
                };
                if path.extension()?.to_str()? != "json" {
                    return None;
                }
                match IEntry::from_file(&path) {
                    Ok(entry) => Some(entry),
                    Err(error) => {
                        eprintln!("Warning: {}", error);
                        None
                    }
                }
            })
            .collect_vec()
            .into())
    }
}
