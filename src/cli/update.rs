use super::*;

pub(super) fn subcommand() -> Command {
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

pub fn process(arg_matches: &ArgMatches, entries: &Entries, today: &SyrDate) -> anyhow::Result<()> {
    let date = match arg_matches.get_one::<String>("date") {
        Some(s) => SyrDate::try_from(s)?,
        None => *today,
    };
    let operation = arg_matches
        .get_one::<String>("operation")
        .ok_or(anyhow!("Failed to parse operation to string"))?;

    let name = arg_matches
        .get_one::<String>("entry")
        .ok_or(anyhow!("Failed to parse entry to string"))?;

    let Some(mut entry) = entries.choose(&name.to_uppercase(), IndexOptions::Indexed) else {
        return Ok(());
    };

    let hour_diff: f64 = *arg_matches.get_one::<f64>("hour").unwrap_or(&0.0);
    let minute_diff: f64 = *arg_matches.get_one::<f64>("minute").unwrap_or(&0.0);
    let second_diff: f64 = *arg_matches.get_one::<f64>("second").unwrap_or(&0.0);
    let total_diff: f64 = hour_diff * 3600.0 + minute_diff * 60.0 + second_diff

    if ["add", "plus", "incr", "increase"]
        .iter()
        .any(|s| *s == operation)
    {
        let tmp = sec_to_pretty_string(entry.get_block_duration(&date));
        entry.increase_bloc_duration(&date, total_diff);
        entry.save()?;
        println!(
            "{}  :  {} {} {}",
            &date,
            &tmp,
            "――>".green(),
            sec_to_pretty_string(entry.get_block_duration(&date))
        )
    } else if ["sub", "rm", "rem", "remove", "minus", "decr", "decrease"]
        .iter()
        .any(|s| *s == operation)
    {
        let tmp = sec_to_pretty_string(entry.get_block_duration(&date));
        entry.decrease_bloc_duration(&date, total_diff);
        entry.save()?;
        println!(
            "{}  :  {} {} {}",
            &date,
            &tmp,
            "――>".red(),
            sec_to_pretty_string(entry.get_block_duration(&date))
        )
    } else {
        return Err(anyhow!("Unkown operation"));
    }

    Ok(())
}
