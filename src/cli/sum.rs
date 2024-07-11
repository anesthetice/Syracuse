use super::*;

pub(super) fn sum_subcommand() -> Command {
    Command::new("sum")
        .aliases(["total", "tally"])
        .about("Sums up the time tracked by entries")
        .long_about("This subcommand is used to sum up the time tracked by specified entries across a span of dates\naliases: 'total', 'tally'")
        .arg(
            Arg::new("exclude")
                .help("entry/entries to exclude")
                .required(false)
                .short('x')
                .long("exclude")
                .num_args(1..20)
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("explicit")
                .help("breaks down each entry's contribution to the total time")
                .required(false)
                .short('e')
                .short_alias('f')
                .long("explicit")
                .alias("full")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("days-back")
                .help("number of days back included")
                .short('d')
                .long("days")
                .alias("days-back")
                .value_parser(value_parser!(usize))
                .action(ArgAction::Set)
                .group("logic-group")
        )
        .arg(
            Arg::new("start-date")
                .help("start date")
                .short('s')
                .long("start")
                .alias("start-date")
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("end-date")
                .help("end date")
                .short('l')
                .long("end")
                .alias("end-date")
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("all")
                .help("everything everywhere all at once")
                .short('a')
                .long("all")
                .alias("EEAATO")
                .group("all-group")
                .action(ArgAction::SetTrue)
        )
        .group(
            ArgGroup::new("logic-group")
                .conflicts_with("start-date")
        )
        .group(
            ArgGroup::new("all-group")
                .conflicts_with_all(["days-back", "start-date", "end-date"])
        )
}

pub fn process_sum_subcommand(
    arg_matches: &ArgMatches,
    entries: &Entries,
    today: &SyrDate,
) -> anyhow::Result<ProcessOutput> {
    let Some(arg_matches) = arg_matches.subcommand_matches("sum") else {
        return Ok(PO::Continue(None));
    };

    let entries: Vec<&Entry> = match arg_matches.get_many::<String>("exclude") {
        Some(entry_match) => {
            let excluded: Vec<Entry> = entry_match
                .flat_map(|s| entries.choose(s.to_uppercase().as_str(), IndexOptions::All))
                .collect();
            entries
                .iter()
                .filter(|entry| {
                    for other in excluded.iter() {
                        if other == *entry {
                            return false;
                        }
                    }
                    true
                })
                .collect()
        }
        None => entries.as_inner(),
    };

    if arg_matches.get_flag("all") {
        let mut total_hours: f64 = 0.0;
        for entry in entries.iter() {
            let hours = entry.get_block_duration_total_as_hours();
            total_hours += hours;
            if arg_matches.get_flag("explicit") && hours != 0.0 {
                println!("{:<15} :   {:.3}", entry.get_name(), hours)
            }
        }

        if arg_matches.get_flag("explicit") {
            println!(
                "\n{} {}",
                "――>".green(),
                format!("{:.3}", total_hours).bold()
            );
        } else {
            println!("{}", format!("{:.3}", total_hours).bold());
        }

        return Ok(PO::Terminate);
    }

    let mut dates: Vec<SyrDate> = Vec::new();

    // days-back + specified end-date or not
    if let Some(num) = arg_matches.get_one::<usize>("days-back") {
        // starts off as end date but will become the start date therefore it's already called start date
        let mut start_date = match arg_matches.get_one::<String>("end-date") {
            Some(string) => SyrDate::try_from(string.as_str()).unwrap_or(*today),
            None => *today,
        };
        dates.push(start_date);
        for _ in 0..*num {
            start_date = start_date
                .previous_day()
                .ok_or(crate::error::Error {})
                .context("invalid number of days back, this is highly unlikely")?
                .into();
            dates.push(start_date);
        }
    }
    // start-date + specified end-date or not
    else {
        let Some(start_date) = arg_matches.get_one::<String>("start-date") else {
            Err(error::Error {}).context("failed to parse starting date as string")?
        };
        let mut start_date = SyrDate::try_from(start_date.as_str())?;

        let end_date = match arg_matches.get_one::<String>("end-date") {
            Some(string) => SyrDate::try_from(string.as_str()).unwrap_or(*today),
            None => *today,
        };

        if start_date > end_date {
            Err(error::Error {}).context("starting date is larger than ending date")?
        }

        dates.push(start_date);
        while start_date < end_date {
            start_date = start_date
                .next_day()
                .ok_or(crate::error::Error{})
                .context("could not advance date while trying to reach specified end date, this is in theory impossible")?
                .into();
            dates.push(start_date);
        }
    }

    let mut total_hours: f64 = 0.0;
    for entry in entries.iter() {
        let hours = dates
            .iter()
            .map(|date| entry.get_block_duration(date))
            .filter(|x| *x != 0)
            .fold(0_f64, |acc, x| acc + x as f64 / 3_600_000_000_000.0_f64);
        total_hours += hours;
        if arg_matches.get_flag("explicit") && hours != 0.0 {
            println!("{:<15} :   {:.3}", entry.get_name(), hours)
        }
    }

    if arg_matches.get_flag("explicit") {
        println!(
            "\n{} {}",
            "――>".green(),
            format!("{:.3}", total_hours).bold()
        );
    } else {
        println!("{}", format!("{:.3}", total_hours).bold());
    }

    Ok(PO::Terminate)
}
