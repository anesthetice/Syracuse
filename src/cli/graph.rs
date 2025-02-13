use super::*;

pub(super) fn subcommand() -> Command {
    Command::new("graph")
        .about("Creates a graph")
        .long_about("This subcommand is used to graph the entries within a specified time frame")
        .arg(
            Arg::new("days-back")
                .help("number of days back included")
                .short('d')
                .long("days")
                .alias("days-back")
                .value_parser(value_parser!(usize))
                .action(ArgAction::Set)
                .group("logic-group"),
        )
        .arg(
            Arg::new("start-date")
                .help("start date")
                .short('s')
                .long("start")
                .alias("start-date")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("end-date")
                .help("end date")
                .short('e')
                .long("end")
                .alias("end-date")
                .action(ArgAction::Set),
        )
        .group(ArgGroup::new("logic-group").conflicts_with("start-date"))
}

pub fn process(arg_matches: &ArgMatches, entries: Entries, today: &SyrDate) -> Result<()> {
    let date_span: SyrSpan = {
        // days-back + specified end-date or not
        if let Some(num) = arg_matches.get_one::<usize>("days-back") {
            let end_date = match arg_matches.get_one::<String>("end-date") {
                Some(string) => SyrDate::try_from(string).unwrap_or(*today),
                None => *today,
            };
            SyrSpan::from_end_and_days_back(*end_date, *num as i64)
        }
        // start-date + specified end-date or not
        else if let Some(start_date) = arg_matches.get_one::<String>("start-date") {
            let start_date = SyrDate::try_from(start_date.as_str())?;

            let end_date = match arg_matches.get_one::<String>("end-date") {
                Some(string) => SyrDate::try_from(string.as_str()).unwrap_or(*today),
                None => *today,
            };

            if start_date > end_date {
                bail!("Start date is more recent than end date");
            }
            SyrSpan::from_start_and_end(*start_date, *end_date)
        } else {
            bail!("Invalid subcommand usage");
        }
    };

    crate::data::graphing::graph(entries, date_span)
}
