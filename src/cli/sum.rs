use crate::data::syrtime::syrspan::SyrSpan;

use super::*;

pub(super) fn subcommand() -> Command {
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

pub fn process(arg_matches: &ArgMatches, entries: &Entries, today: &SyrDate) -> anyhow::Result<()> {
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

        return Ok(());
    }

    let dates: Vec<SyrDate> = {
        // days-back + specified end-date or not
        if let Some(num) = arg_matches.get_one::<usize>("days-back") {
            let mut end_date = match arg_matches.get_one::<String>("end-date") {
                Some(string) => SyrDate::try_from(string).unwrap_or(*today),
                None => *today,
            };
            SyrSpan::from_end_and_days_back(*end_date, *num as i64)
                .into_iter()
                .collect()
        }
        // start-date + specified end-date or not
        else if let Some(start_date) = arg_matches.get_one::<String>("start-date") {
            let mut start_date = SyrDate::try_from(start_date.as_str())?;

            let end_date = match arg_matches.get_one::<String>("end-date") {
                Some(string) => SyrDate::try_from(string.as_str()).unwrap_or(*today),
                None => *today,
            };

            if start_date > end_date {
                Err(anyhow!("Start date is more recent than end date"))?
            }
            SyrSpan::from_start_and_end(*start_date, *end_date)
                .into_iter()
                .collect()
        } else {
            return Err(anyhow!("Invalid subcommand usage"));
        }
    };

    let mut total_hours: f64 = 0.0;
    for entry in entries.iter() {
        let hours = dates
            .iter()
            .map(|date| entry.get_block_duration(date))
            .filter(|x| *x != 0.0)
            .fold(0_f64, |acc, x| acc + x as f64 / 3600.0);
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

    Ok(())
}
