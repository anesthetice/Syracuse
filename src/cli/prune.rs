use super::*;

pub(super) fn prune_subcommand() -> Command {
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

pub fn process_prune_subcommand(
    arg_matches: &ArgMatches,
    mut entries: Entries,
) -> anyhow::Result<ProcessOutput> {
    let Some(arg_matches) = arg_matches.subcommand_matches("prune") else {
        return Ok(PO::Continue(Some(entries)));
    };
    let Some(cutoff_date) = arg_matches.get_one::<String>("date") else {
        Err(anyhow::anyhow!("Failed to parse date as string"))?
    };

    let cutoff_date = SyrDate::try_from(cutoff_date.as_str())?;
    let mut sum: usize = 0;
    for entry in entries.iter_mut() {
        sum += entry.prune(&cutoff_date)?;
    }
    info!(
        "{}",
        format!("{} {} pruned", sum, if sum == 1 { "bloc" } else { "blocs" }).bold()
    );

    Ok(PO::Terminate)
}
