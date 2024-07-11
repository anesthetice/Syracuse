use super::*;

pub(super) fn reindex_subcommand() -> Command {
    Command::new("reindex")
        .about("Reindexes a specified entry")
        .long_about(
            "This subcommand is used to reindex a specified entry that was previously unindexed",
        )
        .arg(
            Arg::new("entry")
                .index(1)
                .required(true)
                .help("entry to unindex")
                .action(ArgAction::Set),
        )
}

pub fn process_reindex_subcommand(
    arg_matches: &ArgMatches,
    entries: &Entries,
) -> anyhow::Result<ProcessOutput> {
    let Some(arg_matches) = arg_matches.subcommand_matches("reindex") else {
        return Ok(PO::Continue(None));
    };

    let Some(entry_match) = arg_matches.get_one::<String>("entry") else {
        Err(error::Error {}).context("failed to parse entry as string")?
    };
    let Some(mut entry) =
        entries.choose(entry_match.to_uppercase().as_str(), IndexOptions::Unindexed)
    else {
        return Ok(PO::Terminate);
    };

    entry.inverse_indexability()?;
    info!("reindexed '{}'", entry.get_name());

    Ok(PO::Terminate)
}
