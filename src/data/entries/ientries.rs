use super::Entries;
use crate::data::IEntry;
use color_eyre::eyre;
use std::path::Path;

pub type IEntries = Entries<true>;

impl IEntries {
    pub fn load_from_path(path: &Path) -> eyre::Result<Self> {
        std::fs::read_dir(path)?
            .filter_map(|res| {
                if let Ok(filepath) = res
                    .inspect_err(|err| eprintln!("Warning: {}", err))
                    .map(|de| de.path())
                    && filepath.extension()?.to_str()? == "json"
                {
                    Some(IEntry::load_from_file(&filepath))
                } else {
                    None
                }
            })
            .collect::<eyre::Result<Vec<IEntry>>>()
            .map(Into::into)
    }
}
