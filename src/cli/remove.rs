use super::*;

pub(super) fn remove_subcommand() -> Command {
    Command::new("remove")
        .aliases(["rm", "delete", "del"])
        .about("Remove a single entry")
        .long_about("This subcommand is used to remove a single entry at a time from syracuse.json\naliases: 'rm', 'delete', 'del'")
        .arg(
            Arg::new("entry")
                .index(1)
                .required(true)
                .help("entry to remove")
                .action(ArgAction::Set),
        )
}

pub fn process_remove_subcommand(
    arg_matches: &ArgMatches,
    entries: &Entries,
) -> anyhow::Result<ProcessOutput> {
    let Some(arg_matches) = arg_matches.subcommand_matches("remove") else {
        return Ok(PO::Continue(None));
    };
    let Some(entry_match) = arg_matches.get_one::<String>("entry") else {
        Err(error::Error {}).context("failed to parse entry as string")?
    };

    let Some(entry) = entries.choose(entry_match.to_uppercase().as_str(), IndexOptions::Indexed)
    else {
        return Ok(PO::Terminate);
    };
    entry.delete()?;
    Ok(PO::Terminate)
}
