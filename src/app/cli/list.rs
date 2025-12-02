use super::*;

pub(super) fn subcommand() -> Command {
    Command::new("list")
        .visible_alias("ls")
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

impl App {
    pub(in crate::app) fn process_list(&self, arg_matches: &ArgMatches) -> Result<()> {
        let mut i_flag = arg_matches.get_flag("indexed");
        let u_flag = arg_matches.get_flag("unindexed");
        if !i_flag && !u_flag {
            i_flag = true;
        }

        let extra_flag = arg_matches.get_flag("extra");

        self.get_anyentries()
            .into_iter()
            .filter(|ae| (ae.indexed && i_flag) || (!ae.indexed && u_flag))
            .for_each(|entry| {
                println!(
                    "• {}",
                    if !extra_flag {
                        entry.display()
                    } else {
                        entry.display().show_blocks()
                    }
                )
            });

        Ok(())
    }
}
