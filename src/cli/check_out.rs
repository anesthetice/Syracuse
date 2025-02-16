use super::*;

pub(super) fn subcommand() -> Command {
    Command::new("check-out")
        .about("Check-out an entry")
        .long_about("This subcommand is used to check-out the previously checked-in entry, adding the difference in time to the count\naliases: 'cout'")
        .alias("cout")
        .arg(
            Arg::new("cancel")
                .help("Does not add the difference in time to the count")
                .long("cancel")
                .required(false)
                .action(ArgAction::SetTrue)
                .exclusive(true),
        )
        .arg(
            Arg::new("check")
                .help("Displays the current difference in time")
                .short('c')
                .long("check")
                .required(false)
                .action(ArgAction::SetTrue)
                .exclusive(true),
        )
}

pub fn process(arg_matches: &ArgMatches, entries: &Entries, today: &SyrDate) -> Result<()> {
    let filepaths = std::fs::read_dir(Dirs::get().data_dir())?
        .filter_map(|filepath| {
            let filepath = match filepath {
                Ok(e) => e,
                Err(err) => {
                    eprintln!("Warning: {}", err);
                    return None;
                }
            }
            .path();

            if filepath.extension()?.to_str()? == "cin" {
                Some(filepath)
            } else {
                None
            }
        })
        .collect_vec();
    {};

    let filepath = match filepaths.len() {
        0 => bail!("Failed to find a single checked-in entry"),
        1 => filepaths[0].clone(),
        _ => bail!("Multiple checked-in entries found"),
    };

    if arg_matches.get_flag("cancel") {
        std::fs::remove_file(&filepath)?;
        return Ok(());
    }

    let mut buffer: Vec<u8> = Vec::new();
    std::fs::OpenOptions::new().read(true).open(&filepath)?.read_to_end(&mut buffer)?;
    let timestamp: jiff::Timestamp = ijson::from_value(&serde_json::from_slice(&buffer)?)?;
    let elapsed = jiff::Timestamp::now().since(timestamp)?.abs().total(jiff::Unit::Second)?;

    if arg_matches.get_flag("check") {
        println!("{} {}", ARROW.green(), stps(elapsed));
        return Ok(());
    }

    let name = filepath.file_stem().ok_or_eyre("Invalid file name")?.to_str().ok_or_eyre("Invalid file name")?;
    let mut entry = entries
        .iter()
        .find(|entry| entry.name == name)
        .ok_or_eyre("Failed to find an entry that matches the checked-in name")?
        .clone();

    let past = entry.get_bloc_duration(today);
    entry.increase_bloc_duration(today, elapsed);
    println!("{} | {} {} {}", today, stps(past), ARROW.green(), stps(entry.get_bloc_duration(today)));
    entry.save()?;
    std::fs::remove_file(&filepath)?;

    Ok(())
}
