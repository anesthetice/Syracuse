use super::*;

pub(super) fn subcommand() -> Command {
    Command::new("check-out")
        .about("Check-out an entry")
        .long_about("This subcommand is used to check-out a previously checked-in entry, the time betweeen the check-in and the check-out is added\naliases: 'cout'")
        .alias("cout")
        .arg(
            Arg::new("ignore")
                .help("prevents the entry from being updated")
                .short('i')
                .long("ignore")
                .required(false)
                .action(ArgAction::SetTrue)
        )
}

pub fn process(arg_matches: &ArgMatches, entries: &Entries, today: &SyrDate) -> anyhow::Result<()> {
    // checks that there is a single .cin entry
    let filepaths = std::path::Path::read_dir(Dirs::get().data_dir())?
        .filter_map(|filepath| {
            let filepath = match filepath {
                Ok(e) => e,
                Err(err) => {
                    log::warn!("{}", err);
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
        0 => return Err(anyhow!("Failed to find a single checked-in entry")),
        1 => filepaths[0].clone(),
        _ => return Err(anyhow!("Multiple checked-in entries found (manual fix)")),
    };
    let name = filepath
        .file_stem()
        .ok_or(anyhow!("Invalid file name"))?
        .to_str()
        .ok_or(anyhow!("Invalid file name"))?;

    let mut buffer: Vec<u8> = Vec::new();
    std::fs::OpenOptions::new()
        .read(true)
        .open(&filepath)?
        .read_to_end(&mut buffer)?;
    let timestamp: jiff::Timestamp = ijson::from_value(&serde_json::from_slice(&buffer)?)?;
    let elapsed = jiff::Timestamp::now()
        .since(timestamp)?
        .abs()
        .total(jiff::Unit::Second)?;

    if !arg_matches.get_flag("ignore") {
        let mut entry = entries
            .iter()
            .find(|entry| entry.get_name() == name)
            .ok_or(anyhow!(
                "No collected entries match the name of the checked-in entry"
            ))?
            .clone();

        let tmp = stps(entry.get_bloc_duration(today));
        entry.increase_bloc_duration(today, elapsed);
        print_datearrow(today, tmp, stps(entry.get_bloc_duration(today)), "green");
        entry.save()?;
    }

    log::debug!(
        "Attempting to delete check-in file '{}'",
        filepath.display()
    );
    std::fs::remove_file(&filepath)?;

    Ok(())
}
