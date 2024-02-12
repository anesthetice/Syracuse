mod cli;
mod data;
use data::internal::{Blocs, Entries, Entry};

fn main() {
    #[cfg(not(debug_assertions))]
    {
        let current_path = std::env::current_dir().expect("Failed to get current path from which Syracuse (binary) is being run");
        let bin_filepath = std::env::current_exe().expect("Failed to get the filepath of Syracuse (binary)");
        let bin_path = bin_filepath.parent().expect("failed to get the parent directory of Syracuse (binary)");
        
        if current_path != bin_path {
            println!("[ERROR] Syracuse (binary) must be run from the same path as itself");
            return;
        }
    }

    {
        use std::path::PathBuf;
        let data_filepath: PathBuf = PathBuf::from("syracuse.json");
        let backups_path: PathBuf = PathBuf::from("./backups");
        if !data_filepath.exists() {
            let entries = Entries::default();
            entries.save().unwrap();
        }
        if !backups_path.exists() {
            std::fs::create_dir(backups_path).unwrap();
        }
    }
    
    let mut entries = Entries::load().unwrap();
    entries.backup().unwrap();

    let command = cli::cli();
    let matches = command.get_matches();

    if let Some(argmatches) = matches.subcommand_matches("add") {
        if let Some(mat) = argmatches.get_many::<String>("entry") {
            let names: Vec<String> = mat.map(|string| {string.to_uppercase()}).collect();
            let mut valid: bool = true;
            'outer:
            for name in names.iter() {
                for entry in entries.iter() {
                    if entry.is_name(name) {valid=false; break 'outer;}
                }
            }
            if !valid {return;}

            entries.push(Entry::new(names, Blocs::default()));
            entries.save().unwrap();
        }
    }
}