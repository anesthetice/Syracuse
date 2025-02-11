use super::*;

pub(super) fn subcommand() -> Command {
    Command::new("check-in")
        .about("Check-in an entry")
        .long_about("This subcommand is used to check-in a specified entry, only a single entry at a time can be checked-in\naliases: 'cin'")
        .alias("cin")
        .arg(
            Arg::new("entry")
                .help("entry to check-in")
                .index(1)
                .required(true)
                .action(ArgAction::Set)
        )
}

pub fn process(arg_matches: &ArgMatches, entries: &Entries) -> Result<()> {
    let name = arg_matches
        .get_one::<String>("entry")
        .ok_or_eyre("Failed to parse entry to string")?;

    let Some(entry) = entries.choose(&name.to_uppercase(), IndexOptions::Indexed) else {
        return Ok(());
    };

    // checks that there are no previously checked-in entries
    for filepath in std::path::Path::read_dir(Dirs::get().data_dir())? {
        let filepath = match filepath {
            Ok(e) => e,
            Err(err) => {
                eprintln!("Warning: {}", err);
                continue;
            }
        }
        .path();

        let Some(ext) = filepath.extension() else {
            continue;
        };
        let Some(ext) = ext.to_str() else {
            continue;
        };
        if ext == "cin" {
            bail!(
                "Another entry is already checked-in: '{}'",
                filepath.display()
            );
        }
    }

    let filepath = Dirs::get()
        .data_dir()
        .join(format!("{}.cin", entry.get_name()));

    let timestamp = serde_json::to_vec(&ijson::to_value(jiff::Timestamp::now())?)?;

    std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&filepath)?
        .write_all(&timestamp)?;

    Ok(())
}
