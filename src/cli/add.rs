use color_eyre::eyre::bail;

use super::*;

pub(super) fn subcommand() -> Command {
    Command::new("add")
        .alias("new")
        .about("Add a new entry to syracuse")
        .long_about("This subcommand is used to add a new entry to syracuse, entries are case-insensitive and can have aliases\naliases: 'new'")
        .arg(Arg::new("entry")
                .index(1)
                .num_args(1..10)
                .required(true)
                .help("entry to add")
                .long_help("entry to add\ne.g. 'add math-201 analysis' will add an entry titled 'MATH-201' with the alias 'ANALYSIS'")
                .action(ArgAction::Set)
            )
}

pub fn process(arg_matches: &ArgMatches, entries: &Entries) -> Result<()> {
    let names = arg_matches
        .get_many::<String>("entry")
        .ok_or_eyre("Failed to parse entry/entries to string/strings")?;
    let mut names: Vec<String> = names.map(|s| s.to_uppercase()).collect();

    let separator_characters = config::Config::get().entry_file_name_separtor.as_str();

    for name in names.iter() {
        if name.contains(separator_characters) {
            bail!(
                "Failed to add new entry, '{name}' conflicts with the separator characters: '{separator_characters}'",
            );
        }
    }

    for entry in entries.iter() {
        for name in names.iter() {
            if !entry.is_new_entry_name_valid(name) {
                bail!(
                    "Failed to add new entry, '{name}' conflicts with an existing entry: '{entry}'"
                );
            }
        }
    }

    Entry::new(names.remove(0), names).save()?;
    println!("New entry added");
    Ok(())
}
