use super::*;

pub(super) fn subcommand() -> Command {
    Command::new("add")
        .visible_aliases(["new", "create"])
        .about("Add a new entry to syracuse")
        .long_about("This subcommand is used to add a new entry to syracuse, entries are case-insensitive and can have aliase")
        .arg(Arg::new("entry")
                .index(1)
                .num_args(1..10)
                .required(true)
                .help("The name followed by any potential aliases of the entry to add to Syracuse")
                .long_help("The name followed by any potential aliases of the entry to add to Syracuse\ne.g. 'add math-201 analysis' will add an entry titled 'MATH-201' with the alias 'ANALYSIS'")
                .action(ArgAction::Set)
            )
}

impl App {
    pub(in crate::app) fn process_add(&self, arg_matches: &ArgMatches) -> Result<()> {
        let mut names = arg_matches
            .get_many::<String>("entry")
            .ok_or_eyre("Failed to parse entry/entries to string/strings")?
            .map(|s| s.to_uppercase())
            .collect_vec();

        let separator = IEntry::SEPARATOR;

        if names.iter().any(|name| name.contains(separator)) {
            bail!(
                "Failed to add new entry, as one of the names entered contains the separator characters: `{separator}`",
            );
        }

        if self
            .get_anyentries()
            .iter()
            .any(|entry| names.iter().any(|name| entry.is_new_entry_name_valid(name)))
        {
            bail!("Failed to add new entry, as one of the names conflicts with an existing entry");
        }

        let entry = IEntry::create(names.remove(0), names);
        entry.save_to_default_file(self.dirs.data_dir())?;
        println!("{} Added '{}'", ARROW.green(), entry);
        Ok(())
    }
}
