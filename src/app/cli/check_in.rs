use super::*;

pub(super) fn subcommand() -> Command {
    Command::new("check-in")
        .about("Check-in an entry")
        .long_about("This subcommand is used to check-in the specified entry\naliases: 'cin'")
        .visible_alias("cin")
        .arg(
            Arg::new("entry")
                .help("The name or alias of the entry to check-in")
                .index(1)
                .required(true)
                .action(ArgAction::Set),
        )
}

pub static CIN_FILENAME: &str = "_checked-in.timestamp";

impl App {
    pub(in crate::app) fn process_check_in(&self, arg_matches: &ArgMatches) -> Result<()> {
        let name = arg_matches
            .get_one::<String>("entry")
            .ok_or_eyre("Failed to parse entry to string")?;

        let Some(entry) = self.choose_indexed(&name.to_uppercase()) else {
            return Ok(());
        };

        // Checks that there are no previously checked-in entries
        let cin_filepath = self.dirs.data_dir().join(CIN_FILENAME);

        if cin_filepath.is_file() {
            bail!("An entry is already checked-in.");
        }

        let data = serde_json::to_vec(&(&entry.name, jiff::Timestamp::now()))?;

        std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&cin_filepath)?
            .write_all(&data)?;

        Ok(())
    }
}
