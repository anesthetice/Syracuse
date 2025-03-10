use super::*;

pub(super) fn subcommand() -> Command {
    Command::new("sum")
        .aliases(["total", "tally"])
        .about("Sum up the time tracked by entries")
        .long_about("This subcommand is used to sum up the time tracked by specified entries across a span of dates\naliases: 'total', 'tally'")
        .arg(
            Arg::new("exclude")
                .help("The entry/entries to exclude")
                .required(false)
                .short('x')
                .long("exclude")
                .num_args(1..20)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("days-back")
                .help("The number of days back included")
                .short('d')
                .long("days")
                .alias("days-back")
                .value_parser(value_parser!(usize))
                .action(ArgAction::Set)
                .group("days-back-group"),
        )
        .arg(
            Arg::new("start-date")
                .help("start date")
                .short('s')
                .long("start")
                .alias("start-date")
                .action(ArgAction::Set)
                .value_parser(clap::value_parser!(SyrDate)),
        )
        .arg(
            Arg::new("end-date")
                .help("end date")
                .short('l')
                .long("end")
                .alias("end-date")
                .action(ArgAction::Set)
                .value_parser(clap::value_parser!(SyrDate)),
        )
        .group(ArgGroup::new("days-back-group").conflicts_with("start-date"))
}

pub fn process(arg_matches: &ArgMatches, entries: &Entries, today: &SyrDate) -> Result<()> {
    let entries: Vec<&Entry> = match arg_matches.get_many::<String>("exclude") {
        Some(entry_match) => {
            let excluded: Vec<String> = entry_match
                .flat_map(|s| entries.choose(&s.to_uppercase(), IndexOptions::All).map(|entry| entry.name))
                .collect();

            entries.iter().filter(|entry| !excluded.contains(&entry.name)).collect()
        }
        None => entries.as_inner(),
    };

    let date_span: Vec<SyrDate> = {
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
    }
    .into_iter()
    .collect();

    let pad = entries
        .iter()
        .map(|entry| entry.name.len() + entry.aliases.first().map(|alias| alias.len()).unwrap_or(0))
        .max()
        .unwrap_or(1_usize)
        + 2;

    let total_hours: f64 = entries
        .iter()
        .map(|entry| {
            let hours: f64 = date_span
                .iter()
                .map(|date| entry.get_bloc_duration(date))
                .filter(|x| *x != 0.0)
                .fold(0_f64, |acc, x| acc + x / 3600.0);
            if hours != 0.0 {
                let pad = if !entry.aliases.is_empty() {
                    pad + 8 // .dim() adds 4 bytes to the start and the end of the string
                } else {
                    pad
                };
                println!("{:<width$} : {:.2}", entry.display_name_and_first_alias(), hours, width = pad);
            }
            hours
        })
        .sum();
    println!("{} {} Hours", ARROW.green(), format!("{:.2}", total_hours).bold());
    Ok(())
}
