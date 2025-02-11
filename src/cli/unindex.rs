use super::*;

pub(super) fn subcommand() -> Command {
    Command::new("unindex")
        .about("Unindexes a specified entry")
        .long_about("This subcommand is used to unindex a specified entry, meaning it will not appear within the choice pool")
        .arg(
            Arg::new("entry")
                .index(1)
                .required(true)
                .help("entry to unindex")
                .action(ArgAction::Set),
        )
}

pub fn process(arg_matches: &ArgMatches, entries: &Entries) -> Result<()> {
    let name = arg_matches
        .get_one::<String>("entry")
        .ok_or_eyre("Failed to parse entry to string")?;

    let Some(mut entry) = entries.choose(&name.to_uppercase(), IndexOptions::Indexed) else {
        return Ok(());
    };

    entry.inverse_indexability()
}
