use super::*;

pub(super) fn subcommand() -> Command {
    Command::new("prune")
        .about("Discard all blocs that are less recent than the cutoff date")
        .long_about("This subcommand is used to remove blocs of time that are less recent than the specified cutoff date")
        .arg(
            Arg::new("date")
                .help("The cutoff date for pruning")
                .index(1)
                .required(true)
                .action(ArgAction::Set)
                .value_parser(clap::value_parser!(SyrDate)),
        )
}

pub fn process(arg_matches: &ArgMatches, mut entries: Entries) -> Result<()> {
    let cutoff_date = arg_matches.get_one::<SyrDate>("date").ok_or_eyre("No cutoff date provided")?;
    let mut sum: usize = 0;
    for entry in entries.iter_mut() {
        let _tmp = entry.blocs.len();
        entry.blocs.retain(|key, _| key >= cutoff_date);
        sum += _tmp - entry.blocs.len();
        entry.save()?;
    }
    println!("{} {} {} pruned", ARROW.green(), sum, (if sum == 1 { "bloc" } else { "blocs" }));
    Ok(())
}
