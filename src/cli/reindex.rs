use super::*;

pub(super) fn subcommand() -> Command {
    Command::new("reindex")
        .about("Reindexes a specified entry")
        .long_about("This subcommand is used to reindex a specified unindexed entry")
        .arg(
            Arg::new("entry")
                .index(1)
                .required(true)
                .help("entry to unindex")
                .action(ArgAction::Set),
        )
}

pub fn process(arg_matches: &ArgMatches, entries: &Entries) -> anyhow::Result<()> {
    let name = arg_matches
        .get_one::<String>("entry")
        .ok_or(anyhow!("Failed to parse entry to string"))?;

    let Some(mut entry) = entries.choose(&name.to_uppercase(), IndexOptions::Unindexed) else {
        return Ok(());
    };

    entry.inverse_indexability()
}
