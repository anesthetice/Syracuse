use super::*;

pub(super) fn update_subcommand() -> Command {
    Command::new("update")
        .about("Manually update the time of an entry")
        .long_about("This subcommand is used to manually increase or decrease the time associated with an entry on a given day")
        .arg(
            Arg::new("entry")
                .index(1)
                .required(true)
                .help("entry to update")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("operation")
                .index(2)
                .required(true)
                .help("add or sub")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("date")
                .required(false)
                .help("the targeted date")
                .long_help("the targeted date, defaults to today")
                .short('d')
                .long("date")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("hour")
                .required(false)
                .help("the number of hours to add or subtract")
                .short('t')
                .long("hour")
                .value_parser(value_parser!(f64))
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("minute")
                .required(false)
                .help("the number of minutes to add or subtract")
                .short('m')
                .long("minute")
                .value_parser(value_parser!(f64))
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("second")
                .required(false)
                .help("the number of seconds to add or subtract")
                .short('s')
                .long("second")
                .value_parser(value_parser!(f64))
                .action(ArgAction::Set),
        )
}

pub fn process_update_subcommand(
    arg_matches: &ArgMatches,
    entries: &Entries,
    today: &SyrDate,
) -> anyhow::Result<ProcessOutput> {
    let Some(arg_matches) = arg_matches.subcommand_matches("update") else {
        return Ok(PO::Continue(None));
    };
    let date = match arg_matches.get_one::<String>("date") {
        Some(string) => SyrDate::try_from(string.as_str())?,
        None => *today,
    };
    let Some(operation) = arg_matches.get_one::<String>("operation") else {
        Err(error::Error {}).context("failed to parse operation as string")?
    };
    let Some(entry_match) = arg_matches.get_one::<String>("entry") else {
        Err(error::Error {}).context("failed to parse entry as string")?
    };
    let Some(mut entry) =
        entries.choose(entry_match.to_uppercase().as_str(), IndexOptions::Indexed)
    else {
        return Ok(PO::Terminate);
    };
    let hour_diff: f64 = *arg_matches.get_one::<f64>("hour").unwrap_or(&0.0);
    let minute_diff: f64 = *arg_matches.get_one::<f64>("minute").unwrap_or(&0.0);
    let second_diff: f64 = *arg_matches.get_one::<f64>("second").unwrap_or(&0.0);
    let total_diff: u128 = (hour_diff * 3_600_000_000_000_f64
        + minute_diff * 60_000_000_000_f64
        + second_diff * 1_000_000_000_f64) as u128;

    if ["add", "plus", "incr", "increase"]
        .iter()
        .any(|s| *s == operation)
    {
        let tmp = ns_to_pretty_string(entry.get_block_duration(&date));
        entry.increase_bloc_duration(&date, total_diff);
        entry.save()?;
        println!(
            "{}  :  {} {} {}",
            &date,
            &tmp,
            "――>".green(),
            ns_to_pretty_string(entry.get_block_duration(&date))
        )
    } else if ["sub", "rm", "rem", "remove", "minus", "decr", "decrease"]
        .iter()
        .any(|s| *s == operation)
    {
        let tmp = ns_to_pretty_string(entry.get_block_duration(&date));
        entry.decrease_bloc_duration(&date, total_diff);
        entry.save()?;
        println!(
            "{}  :  {} {} {}",
            &date,
            &tmp,
            "――>".red(),
            ns_to_pretty_string(entry.get_block_duration(&date))
        )
    } else {
        warn!("unknown operation: '{}'", operation);
    }
    Ok(PO::Terminate)
}
