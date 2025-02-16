use super::*;

pub(super) fn subcommand() -> Command {
    Command::new("update-add")
        .aliases(["upadd", "upincr", "upplus"])
        .about("Manually increase the time tracked by an entry")
        .long_about("This subcommand is used to manually increase the time associated with an entry on a given day\naliases: 'upadd', 'upincr', 'upplus'")
        .arg(Arg::new("entry").index(1).required(true).help("The entry to update").action(ArgAction::Set))
        .arg(
            Arg::new("days-back")
                .help("The number of days back to check")
                .short('d')
                .long("days-back")
                .value_parser(value_parser!(usize))
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("date")
                .required(false)
                .help("The target date")
                .long("date")
                .value_parser(value_parser!(SyrDate))
                .action(ArgAction::Set)
                .group("date-group"),
        )
        .arg(
            Arg::new("hours")
                .required(false)
                .help("The number of hours to add")
                .short('t')
                .long("hours")
                .value_parser(value_parser!(f64))
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("minutes")
                .required(false)
                .help("The number of minutes to add")
                .short('m')
                .long("minutes")
                .value_parser(value_parser!(f64))
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("seconds")
                .required(false)
                .help("The number of seconds to add")
                .short('s')
                .long("seconds")
                .value_parser(value_parser!(f64))
                .action(ArgAction::Set),
        )
        .group(ArgGroup::new("date-group").conflicts_with("days-back"))
}

pub fn process(arg_matches: &ArgMatches, entries: &Entries, today: &SyrDate) -> Result<()> {
    let date = {
        if let Some(days_back) = arg_matches.get_one::<usize>("days-back") {
            today.saturating_sub(Span::new().days(i64::try_from(*days_back)?)).into()
        } else if let Some(date) = arg_matches.get_one::<SyrDate>("date") {
            *date
        } else {
            *today
        }
    };

    let name = arg_matches.get_one::<String>("entry").ok_or_eyre("Failed to parse entry to string")?;
    let Some(mut entry) = entries.choose(&name.to_uppercase(), IndexOptions::Indexed) else {
        return Ok(());
    };

    let hour_diff: f64 = *arg_matches.get_one::<f64>("hours").unwrap_or(&0.0);
    let minute_diff: f64 = *arg_matches.get_one::<f64>("minutes").unwrap_or(&0.0);
    let second_diff: f64 = *arg_matches.get_one::<f64>("seconds").unwrap_or(&0.0);
    let total_diff: f64 = hour_diff * 3600.0 + minute_diff * 60.0 + second_diff;

    let past = entry.get_bloc_duration(&date);
    entry.increase_bloc_duration(&date, total_diff);
    entry.save()?;
    println!("{} | {} {} {}", &date, stps(past), ARROW.green(), stps(entry.get_bloc_duration(&date)));

    Ok(())
}
