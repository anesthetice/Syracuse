use super::*;

pub(super) fn subcommand() -> Command {
    Command::new("remove")
        .aliases(["rm", "delete", "del"])
        .about("Remove an entry")
        .long_about("This subcommand is used to remove an entry, by deleting its associated file\naliases: 'rm', 'delete', 'del'")
        .arg(
            Arg::new("entry")
                .index(1)
                .required(true)
                .help("The entry to remove")
                .action(ArgAction::Set),
        )
}

pub fn process(arg_matches: &ArgMatches, entries: &Entries) -> Result<()> {
    let name = arg_matches
        .get_one::<String>("entry")
        .ok_or_eyre("Failed to parse entry to string")?;
    if let Some(entry) = entries.choose(&name.to_uppercase(), IndexOptions::Indexed) {
        entry.delete()?;
    };
    Ok(())
}
