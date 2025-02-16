use super::*;

pub(super) fn subcommand() -> Command {
    Command::new("unindex")
        .about("Unindex one or more entries")
        .long_about("This subcommand is used to unindex one or more specified entries\nUnindexed entries do not show up within the choice pool")
        .arg(
            Arg::new("entries")
                .index(1)
                .required(true)
                .help("The entries to unindex")
                .num_args(1..10)
                .action(ArgAction::Set),
        )
}

pub fn process(arg_matches: &ArgMatches, entries: &Entries) -> Result<()> {
    for mut entry in arg_matches
        .get_many::<String>("entries")
        .ok_or_eyre("Failed to parse entry to string")?
        .filter_map(|name| entries.choose(&name.to_uppercase(), IndexOptions::Indexed))
    {
        entry.inverse_indexability()?;
    }
    Ok(())
}
