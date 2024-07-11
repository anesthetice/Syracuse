use super::*;

pub(super) fn list_subcommand() -> Command {
    Command::new("list")
        .alias("ls")
        .about("List out all entries")
        .long_about("This subcommand is used to list out all entries stored\naliases: 'ls'")
        .arg(
            Arg::new("indexed")
                .short('i')
                .long("indexed")
                .help("lists indexed entries, default behavior")
                .required(false)
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("unindexed")
                .short('u')
                .long("unindexed")
                .help("lists unindex entries")
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
                .help("prints out the data associated with each entry as well")
                .required(false)
                .action(ArgAction::SetTrue),
        )
}

pub fn process_list_subcommand(
    arg_matches: &ArgMatches,
    entries: &Entries,
) -> anyhow::Result<ProcessOutput> {
    let Some(arg_matches) = arg_matches.subcommand_matches("list") else {
        return Ok(PO::Continue(None));
    };

    let entries: Vec<&Entry> = match (
        arg_matches.get_flag("indexed"),
        arg_matches.get_flag("unindexed"),
    ) {
        (true, true) => entries.iter().collect(),
        (true, false) | (false, false) => {
            entries.iter().filter(|entry| entry.is_indexed()).collect()
        }
        (false, true) => entries.iter().filter(|entry| !entry.is_indexed()).collect(),
    };

    match entries.len() {
        0 => println!("No entries found"),
        1 => println!("Found a single entry"),
        n => println!("Found {n} entries:\n"),
    }
    if arg_matches.get_flag("extra") {
        for entry in entries.iter() {
            println!("• {:?}\n", entry)
        }
    } else {
        for entry in entries.iter() {
            println!("• {}\n", entry)
        }
    }
    Ok(PO::Terminate)
}
