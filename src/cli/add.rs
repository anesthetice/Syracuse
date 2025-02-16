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
                .help("The name followed by any potential aliases of the entry to add to Syracuse")
                .long_help("The name followed by any potential aliases of the entry to add to Syracuse\ne.g. 'add math-201 analysis' will add an entry titled 'MATH-201' with the alias 'ANALYSIS'")
                .action(ArgAction::Set)
            )
}

pub fn process(arg_matches: &ArgMatches, entries: &Entries) -> Result<()> {
    let names = arg_matches
        .get_many::<String>("entry")
        .ok_or_eyre("Failed to parse entry/entries to string/strings")?;
    let mut names: Vec<String> = names.map(|s| s.to_uppercase()).collect();

    let separator = config::Config::get().entry_file_name_separtor.as_str();

    if names.iter().any(|name| name.contains(separator)) {
        bail!("Failed to add new entry, one of the names conflicts with the separator '{separator}'",);
    }

    if entries.iter().any(|entry| names.iter().any(|name| entry.is_new_entry_name_valid(name))) {
        bail!("Failed to add new entry, one of the names conflicts with an existing entry.");
    }

    let entry = Entry::create(names.remove(0), names);
    entry.save()?;
    println!("{} Added '{}'", ARROW.green(), entry);
    Ok(())
}
