use super::*;

pub(super) fn subcommand() -> Command {
    Command::new("graph")
        .about("Graphs the stored entries")
        .long_about("This subcommand is used to graph the stored entries within a specified time frame")
        .arg(
            Arg::new("days-back")
                .help("The number of days back")
                .long("The number of days back, if an end-date is not specified then today will be used")
                .short('d')
                .long("days")
                .alias("days-back")
                .value_parser(value_parser!(usize))
                .action(ArgAction::Set)
                .group("days-back-group"),
        )
        .arg(
            Arg::new("start-date")
                .help("The start date")
                .long_help("The start date, if an end-date is not specified then today will be used")
                .short('s')
                .long("start")
                .alias("start-date")
                .action(ArgAction::Set)
                .value_parser(clap::value_parser!(SyrDate)),
        )
        .arg(
            Arg::new("end-date")
                .help("The end date")
                .short('e')
                .long("end")
                .alias("end-date")
                .action(ArgAction::Set)
                .value_parser(clap::value_parser!(SyrDate)),
        )
        .group(ArgGroup::new("days-back-group").conflicts_with("start-date"))
}

pub fn process(arg_matches: &ArgMatches, entries: Entries, today: &SyrDate) -> Result<()> {
    let date_span: SyrSpan = {
        // days-back + specified end-date or not
        if let Some(num) = arg_matches.get_one::<usize>("days-back") {
            let end_date = *arg_matches.get_one::<SyrDate>("end-date").unwrap_or(today);
            SyrSpan::from_end_and_days_back(*end_date, *num as i64)
        }
        // start-date + specified end-date or not
        else if let Some(start_date) = arg_matches.get_one::<SyrDate>("start-date") {
            let end_date = *arg_matches.get_one::<SyrDate>("end-date").unwrap_or(today);

            if *start_date > end_date {
                bail!("Start date is more recent than end date");
            }
            SyrSpan::from_start_and_end(**start_date, *end_date)
        } else {
            bail!("Invalid subcommand usage");
        }
    };

    crate::data::graphing::graph(entries, date_span)
}
