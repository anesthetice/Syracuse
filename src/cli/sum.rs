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

    type CompactOutput<'a> = (Vec<(&'a str, Option<&'a str>, f64)>, usize, f64);
    let (mut bones, pad, total_hours): CompactOutput = entries
        .into_iter()
        .filter_map(|entry| {
            let hours: f64 = date_span
                .iter()
                .filter_map(|date| entry.get_block_duration_opt(date))
                .sum1()
                .map(|val: f64| val / 3600.0)?;
            let name: &str = entry.name.as_str();
            let alias: Option<&str> = entry.aliases.first().map(String::as_str);
            let padding = name.len() + alias.map(str::len).unwrap_or(0) + 2;
            Some((name, alias, hours, padding))
        })
        .fold(
            (Vec::new(), 0, 0.0),
            |(mut output, pad, total_hours), (name, alias, hours, pad_)| {
                output.push((name, alias, hours));
                (output, pad.max(pad_), total_hours + hours)
            },
        );

    bones.sort_by(|a, b| match config::Config::get().sort_option {
        SortOptions::NameAscending => a.0.cmp(b.0),
        SortOptions::NameDescending => b.0.cmp(a.0),
        SortOptions::DurationAscending => a.2.total_cmp(&b.2),
        SortOptions::DurationDescending => b.2.total_cmp(&a.2),
    });

    bones.into_iter().for_each(|(name, alias, hours)| match alias {
        Some(alias) => {
            let title: String = format!("{}; {}", name, alias.dim());
            println!("{:<width$} : {:.2}", title, hours, width = pad + 8);
        }
        None => {
            println!("{:<width$} : {:.2}", name, hours, width = pad);
        }
    });

    println!("{} {} Hours", ARROW.green(), format!("{:.2}", total_hours).bold());
    Ok(())
}
