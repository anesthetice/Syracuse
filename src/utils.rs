// Imports
use color_eyre::Result;
use crossterm::{
    cursor, execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::stdout;

use crate::data::{Entries, Entry};

pub static ARROW: &str = "━━⮞";
pub static ARROWHEAD: &str = "⮞";

pub fn enter_clean_input_mode() {
    let _ = enable_raw_mode().map_err(|err| eprintln!("Warning, Failed to enable raw mode: '{err}'"));
    let _ = execute!(stdout(), cursor::Hide).map_err(|err| eprintln!("Warning, Failed to hide cursor: '{err}'"));
}

pub fn exit_clean_input_mode() {
    let _ = execute!(stdout(), cursor::Show).map_err(|err| eprintln!("Warning: Failed to show cursor: '{err}'"));
    let _ = disable_raw_mode().map_err(|err| eprintln!("Warning: Failed to disable raw mode: '{err}'"));
}

#[cfg(debug_assertions)]
#[allow(unused)]
pub fn update_filename_separator(entries: &Entries, old_separator: &str, new_separator: &str) -> Result<()> {
    for entry in entries.iter() {
        let old_filepath = entry.get_filepath();
        let new_filepath = Entry::get_dirname().join(entry.get_filename().replace(old_separator, new_separator));
        println!("{} → {}", old_filepath.display(), new_filepath.display());
        std::fs::rename(old_filepath, new_filepath)?;
    }
    Ok(())
}

#[cfg(debug_assertions)]
#[allow(unused)]
pub fn convert_from_v2_to_v3(entries: Entries) -> Result<()> {
    for mut entry in entries.into_iter() {
        entry.blocs.iter_mut().for_each(|(_, val)| {
            if *val > 43200.0 {
                *val /= 1e9
            }
        });
        entry.save()?;
    }
    Ok(())
}
