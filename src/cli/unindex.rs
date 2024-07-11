use super::*;

pub(super) fn unindex_subcommand() -> Command {
    Command::new("unindex")
        .about("Unindexes a specified entry")
        .long_about("This subcommand is used to unindex a specified entry, meaning it will not appear within the choice pool for other command")
        .arg(
            Arg::new("entry")
                .index(1)
                .required(true)
                .help("entry to unindex")
                .action(ArgAction::Set),
        )
}

pub fn process_unindex_subcommand(
    arg_matches: &ArgMatches,
    entries: &Entries,
) -> anyhow::Result<ProcessOutput> {
    let Some(arg_matches) = arg_matches.subcommand_matches("unindex") else {
        return Ok(PO::Continue(None));
    };

    let Some(entry_match) = arg_matches.get_one::<String>("entry") else {
        Err(error::Error {}).context("failed to parse entry as string")?
    };
    let Some(mut entry) =
        entries.choose(entry_match.to_uppercase().as_str(), IndexOptions::Indexed)
    else {
        return Ok(PO::Terminate);
    };

    entry.inverse_indexability()?;
    info!("unindexed '{}'", entry.get_name());

    Ok(PO::Terminate)
}
