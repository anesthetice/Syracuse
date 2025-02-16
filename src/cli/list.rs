use super::*;

pub(super) fn subcommand() -> Command {
    Command::new("list")
        .alias("ls")
        .about("List out stored entries")
        .long_about("This subcommand is used to list out stored entries\naliases: 'ls'")
        .arg(
            Arg::new("indexed")
                .short('i')
                .long("indexed")
                .help("Lists indexed entries, default behavior")
                .required(false)
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("unindexed")
                .short('u')
                .long("unindexed")
                .help("Lists unindex entries")
                .required(false)
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("extra")
                .short('e')
                .short_alias('f')
                .long("extra")
                .alias("full")
                .alias("explicit")
                .help("Displays the data associated with each entry")
                .required(false)
                .action(ArgAction::SetTrue),
        )
}

pub fn process(arg_matches: &ArgMatches, entries: &Entries) -> Result<()> {
    let entries: Vec<&Entry> = match (arg_matches.get_flag("indexed"), arg_matches.get_flag("unindexed")) {
        (true, true) => entries.iter().collect(),
        (true, false) | (false, false) => entries.iter().filter(|entry| entry.indexed).collect(),
        (false, true) => entries.iter().filter(|entry| !entry.indexed).collect(),
    };

    if arg_matches.get_flag("extra") {
        for entry in entries.iter() {
            println!("• {:?}", entry)
        }
    } else {
        for entry in entries.iter() {
            println!("• {}", entry)
        }
    }
    Ok(())
}
