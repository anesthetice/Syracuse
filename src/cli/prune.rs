use super::*;

pub(super) fn subcommand() -> Command {
    Command::new("prune")
        .about("Keeps only the blocs younger than a certain date old")
        .long_about(
            "This subcommand is used to remove blocs of time that are older than the provided date",
        )
        .arg(
            Arg::new("date")
                .help("cutoff date for pruning")
                .index(1)
                .required(true)
                .action(ArgAction::Set),
        )
}

pub fn process(arg_matches: &ArgMatches, mut entries: Entries) -> Result<()> {
    let cutoff_date: SyrDate = arg_matches
        .get_one::<String>("date")
        .ok_or_eyre("Failed to parse date as string")?
        .try_into()?;

    let mut sum: usize = 0;
    for entry in entries.iter_mut() {
        sum += entry.prune(&cutoff_date)?;
    }
    println!(
        "{} {} pruned",
        sum,
        (if sum == 1 { "bloc" } else { "blocs" }).bold()
    );

    Ok(())
}
