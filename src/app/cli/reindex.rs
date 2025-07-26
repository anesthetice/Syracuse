use super::*;

pub(super) fn subcommand() -> Command {
    Command::new("reindex")
        .about("Reindex one or more entries")
        .long_about(
            "This subcommand is used to reindex one or more specified entries\nUnlike unindexed entries, indexed entries can appear in the choice pool",
        )
        .arg(
            Arg::new("entries")
                .index(1)
                .required(true)
                .help("The entries to reindex")
                .num_args(1..10)
                .action(ArgAction::Set),
        )
}

pub fn process(arg_matches: &ArgMatches, entries: &Entries) -> Result<()> {
    for mut entry in arg_matches
        .get_many::<String>("entries")
        .ok_or_eyre("Failed to parse entry to string")?
        .filter_map(|name| entries.choose(&name.to_uppercase(), IndexOptions::Unindexed))
    {
        entry.inverse_indexability()?;
    }
    Ok(())
}
