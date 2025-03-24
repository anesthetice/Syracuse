use super::*;

pub(super) fn subcommand() -> Command {
    Command::new("check-in")
        .about("Check-in an entry")
        .long_about("This subcommand is used to check-in the specified entry\naliases: 'cin'")
        .alias("cin")
        .arg(
            Arg::new("entry")
                .help("The name or alias of the entry to check-in")
                .index(1)
                .required(true)
                .action(ArgAction::Set),
        )
}

pub fn process(arg_matches: &ArgMatches, entries: &Entries) -> Result<()> {
    let name = arg_matches
        .get_one::<String>("entry")
        .ok_or_eyre("Failed to parse entry to string")?;

    let Some(entry) = entries.choose(&name.to_uppercase(), IndexOptions::Indexed) else {
        return Ok(());
    };

    // Checks that there are no previously checked-in entries
    std::fs::read_dir(Dirs::get().data_dir())?
        .filter_map(|entry| match entry {
            Ok(e) => Some(e.path()),
            Err(err) => {
                eprintln!("Warning: {}", err);
                None
            }
        })
        .try_for_each(|path| {
            if path.extension().and_then(|ext| ext.to_str()) == Some("cin") {
                bail!("Another entry is already checked-in: '{}'", path.display());
            }
            Ok(())
        })?;

    let filepath = Dirs::get().data_dir().join([&entry.name, ".cin"].concat());

    let timestamp = serde_json::to_vec(&ijson::to_value(jiff::Timestamp::now())?)?;

    std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&filepath)?
        .write_all(&timestamp)?;

    Ok(())
}
