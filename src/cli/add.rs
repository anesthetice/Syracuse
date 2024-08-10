use super::*;

pub(super) fn add_subcommand() -> Command {
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

pub fn process_add_subcommand(
    arg_matches: &ArgMatches,
    entries: &Entries,
) -> anyhow::Result<ProcessOutput> {
    let Some(arg_matches) = arg_matches.subcommand_matches("add") else {
        return Ok(PO::Continue(None));
    };
    let Some(entry_match) = arg_matches.get_many::<String>("entry") else {
        Err(anyhow::anyhow!("Failed to parse entry as string"))?
    };
    let mut names: Vec<String> = entry_match.map(|s| s.to_uppercase()).collect();

    let separator_characters = config::Config::get().entry_file_name_separtor.as_str();
    for name in names.iter() {
        if name.contains(separator_characters) {
            Err(anyhow::anyhow!(
                "Failed to add new entry, '{name}' conflicts with the separator characters: '{separator_characters}'",
            ))?;
        }
    }

    for entry in entries.iter() {
        for name in names.iter() {
            if !entry.check_new_entry_name_validity(name) {
                Err(anyhow::anyhow!(
                    "Failed to add new entry, '{name}' conflict with an existing entry: '{entry}'"
                ))?
            }
        }
    }

    Entry::new(names.remove(0), names).save()?;
    Ok(PO::Terminate)
}
