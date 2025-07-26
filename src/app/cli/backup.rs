use super::*;

pub(super) fn subcommand() -> Command {
    Command::new("backup")
        .about("Create a backup of all entries")
        .long_about("This subcommand is used to backup all entries to a directory specified in the configuration file or passed as an argument")
        .arg(Arg::new("path").help("The path to back up to").index(1).action(ArgAction::Set))
}

pub fn process(arg_matches: &ArgMatches, entries: &Entries, dt: &DateTime) -> Result<()> {
    let folder = format!(
        "{:0>4}_{:0>2}_{:0>2}-{:0>2}_{:0>2}_{:0>2}/",
        dt.year(),
        dt.month(),
        dt.day(),
        dt.hour(),
        dt.minute(),
        dt.second(),
    );

    let path = match arg_matches.get_one::<String>("path") {
        Some(string) => PathBuf::from(string),
        None => PathBuf::from(config::Config::get().backup_path.as_str()),
    }
    .join(folder);

    std::fs::create_dir(&path).wrap_err("Failed to create backup directory")?;

    for entry in entries.iter() {
        if let Err(error) = entry.save_to_file(&path.join(entry.get_filestem() + ".json")) {
            eprintln!("Warning: Failed to back up an entry, '{error}'")
        }
    }

    println!(
        "{} Created backup at '{}'",
        ARROW.green(),
        path.canonicalize().unwrap_or(path).display()
    );
    Ok(())
}
