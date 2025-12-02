use super::*;

pub(super) fn subcommand() -> Command {
    Command::new("check-out")
        .about("Check-out an entry")
        .long_about(
            "This subcommand is used to check-out the previously checked-in entry, adding the difference in time to the count\naliases: 'cout'",
        )
        .visible_alias("cout")
        .arg(
            Arg::new("cancel")
                .help("Does not add the difference in time to the time tracked by the entry")
                .long("cancel")
                .required(false)
                .action(ArgAction::SetTrue)
                .exclusive(true),
        )
        .arg(
            Arg::new("check")
                .help("Displays the current difference in time")
                .short('c')
                .long("check")
                .required(false)
                .action(ArgAction::SetTrue)
                .exclusive(true),
        )
}

impl App {
    pub(in crate::app) fn process_check_out(
        &self,
        arg_matches: &ArgMatches,
        today: &SyrDate,
    ) -> Result<()> {
        let cin_filepath = self.dirs.data_dir().join(CIN_FILENAME);

        if !cin_filepath.is_file() {
            bail!("No entry is currently checked-in.")
        }

        let mut buffer: Vec<u8> = Vec::new();
        std::fs::OpenOptions::new()
            .read(true)
            .open(&cin_filepath)?
            .read_to_end(&mut buffer)?;

        let (cin_entry_name, timestamp): (String, jiff::Timestamp) =
            serde_json::from_slice(&buffer)?;

        let elapsed = jiff::Timestamp::now()
            .since(timestamp)?
            .abs()
            .total(jiff::Unit::Second)?;

        if arg_matches.get_flag("cancel") {
            println!("{} {}", ARROW.red(), elapsed.ms_str());
            std::fs::remove_file(&cin_filepath)?;
            return Ok(());
        } else if arg_matches.get_flag("check") {
            println!("{} {}", ARROW.yellow(), elapsed.ms_str());
            return Ok(());
        }

        let mut entry = self
            .ientries
            .iter()
            .find(|&entry| entry.name == cin_entry_name)
            .ok_or_eyre("Failed to find an entry that matches the checked-in name")?
            .clone();

        let past = entry.get_bloc_duration(today);
        entry.increase_bloc_duration(today, elapsed);
        println!(
            "{}\n{} {} {}",
            entry.display(),
            past.s_str(),
            ARROW.green(),
            (past + elapsed).s_str(),
        );
        entry.save_to_default_file(self.dirs.data_dir())?;
        std::fs::remove_file(&cin_filepath)?;

        Ok(())
    }
}
