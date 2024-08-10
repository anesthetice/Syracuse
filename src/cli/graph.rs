use super::*;

pub(super) fn graph_subcommand() -> Command {
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

pub fn process_graph_subcommand(
    arg_matches: &ArgMatches,
    entries: Entries,
    today: &SyrDate,
) -> anyhow::Result<ProcessOutput> {
    let Some(arg_matches) = arg_matches.subcommand_matches("graph") else {
        return Ok(PO::Continue(Some(entries)));
    };

    // days-back + specified end-date or not
    if let Some(num) = arg_matches.get_one::<usize>("days-back") {
        // starts off as end date but will become the start date therefore it's already called start date
        let mut start_date = match arg_matches.get_one::<String>("end-date") {
            Some(string) => SyrDate::try_from(string.as_str()).unwrap_or(*today),
            None => *today,
        };
        for _ in 0..*num {
            start_date = start_date
                .previous_day()
                .ok_or(anyhow::anyhow!(
                    "invalid number of days back, this is highly unlikely"
                ))?
                .into();
        }
        graphing::graph(entries, start_date, *today)?;
        Ok(PO::Terminate)
    }
    // start-date + specified end-date or not
    else {
        let Some(start_date) = arg_matches.get_one::<String>("start-date") else {
            Err(error::Error {}).context("failed to parse starting date as string")?
        };
        let start_date = SyrDate::try_from(start_date.as_str())?;

        let end_date = match arg_matches.get_one::<String>("end-date") {
            Some(string) => SyrDate::try_from(string.as_str()).unwrap_or(*today),
            None => *today,
        };

        if start_date > end_date {
            Err(error::Error {}).context("starting date is larger than ending date")?
        }

        crate::data::graphing::graph(entries, start_date, end_date)?;
        Ok(PO::Terminate)
    }
}
