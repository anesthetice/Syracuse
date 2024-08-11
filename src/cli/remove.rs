use super::*;

pub(super) fn subcommand() -> Command {
    Command::new("remove")
        .aliases(["rm", "delete", "del"])
        .about("Remove a single entry")
        .long_about("This subcommand is used to remove an entry by deleting it's associated file\naliases: 'rm', 'delete', 'del'")
        .arg(
            Arg::new("entry")
                .index(1)
                .required(true)
                .help("entry to remove")
                .action(ArgAction::Set),
        )
}

pub fn process(arg_matches: &ArgMatches, entries: &Entries) -> anyhow::Result<()> {
    let name = arg_matches
        .get_one::<String>("entry")
        .ok_or(anyhow!("Failed to parse entry to string"))?;

    if let Some(entry) = entries.choose(&name.to_uppercase(), IndexOptions::Indexed) {
        entry.delete()?;
    };

    Ok(())
}
