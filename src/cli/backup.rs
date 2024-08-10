use super::*;

pub(super) fn backup_subcommand() -> Command {
    Command::new("backup")
        .about("Backup entries")
        .long_about("This subcommand is used to backup all entries to a directory specified in the configuration file or directly provided by the user")
        .arg(
            Arg::new("path")
                .help("specified path")
                .index(1)
                .action(ArgAction::Set)
        )
}

pub fn process_backup_subcommand(
    arg_matches: &ArgMatches,
    entries: &Entries,
    today_datetime: &OffsetDateTime,
) -> anyhow::Result<ProcessOutput> {
    let Some(arg_matches) = arg_matches.subcommand_matches("backup") else {
        return Ok(PO::Continue(None));
    };
    let folder = format!(
        "{:0>4}_{:0>2}_{:0>2}-{:0>2}_{:0>2}_{:0>2}/",
        today_datetime.year(),
        today_datetime.month(),
        today_datetime.day(),
        today_datetime.hour(),
        today_datetime.minute(),
        today_datetime.second(),
    );

    let path = match arg_matches.get_one::<String>("path") {
        Some(string) => PathBuf::from(string),
        None => PathBuf::from(config::Config::get().backup_path.as_str()),
    }
    .join(folder);

    if let Err(error) = std::fs::create_dir(&path) {
        warn!("Failed to create '{}',: '{}'", path.display(), error);
        return Ok(PO::Terminate);
    }
    info!("Backing up to: '{}'", &path.display());

    entries.backup(path);
    Ok(PO::Terminate)
}
