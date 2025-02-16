use super::*;

pub(super) fn subcommand() -> Command {
    Command::new("today")
        .about("Display the time tracked today")
        .long_about("This subcommand is used to display the sum of the time tracked by every single entry for today")
        .arg(
            Arg::new("explicit")
                .help("Breaks down each entry's contribution to the total time")
                .required(false)
                .short('e')
                .short_alias('f')
                .long("explicit")
                .alias("full")
                .action(ArgAction::SetTrue),
        )
        .arg(Arg::new("yesterday").required(false).short('y').long("yesterday").action(ArgAction::SetTrue))
        .arg(
            Arg::new("days-back")
                .help("The number of days back to check")
                .short('d')
                .short_alias('p')
                .long("days-back")
                .value_parser(value_parser!(usize))
                .action(ArgAction::Set),
        )
}

pub fn process(arg_matches: &ArgMatches, entries: &Entries, today: &SyrDate) -> Result<()> {
    let date: SyrDate = {
        let mut date = **today;
        if arg_matches.get_flag("yesterday") {
            date = date.yesterday()?;
        } else if let Some(val) = arg_matches.get_one::<usize>("previous") {
            date = date.saturating_sub(Span::new().days(i64::try_from(*val)?));
        }
        date.into()
    };

    if arg_matches.get_flag("explicit") {
        let sum = entries
            .iter()
            .map(|entry| {
                let duration = entry.get_bloc_duration(&date);
                if duration != 0.0 {
                    println!("{:<15} :   {}", entry.name, stps(duration))
                }
                duration
            })
            .sum();
        println!("{} {}", ARROW.green(), stps(sum).bold());
    } else {
        let sum = entries.iter().map(|entry| entry.get_bloc_duration(&date)).sum();
        println!("{} {}", ARROW.green(), stps(sum).bold());
    }

    Ok(())
}
