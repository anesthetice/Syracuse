use super::*;

pub(super) fn subcommand() -> Command {
    Command::new("today")
        .about("Display the time tracked today")
        .long_about("This subcommand is used to display the sum of the time tracked by every single entry for today")
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
            Arg::new("yesterday")
                .required(false)
                .short('y')
                .long("yesterday")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("previous")
                .required(false)
                .short('p')
                .long("previous")
                .alias("prev")
                .value_parser(value_parser!(usize))
                .action(ArgAction::Set)
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

    let sum = {
        if arg_matches.get_flag("explicit") {
            entries
                .iter()
                .map(|entry| {
                    let duration = entry.get_bloc_duration(&date);
                    // 15 seems reasonable, I could check the length of every entry's name and get a better estimation, but even that would not be perfect, I would have to count the valid grapheme clusters which adds a lot of complexity, to what I itend as simple padding
                    if duration != 0.0 {
                        println!("{:<15} :   {}", entry.get_name(), stps(duration))
                    }
                    duration
                })
                .sum()
        } else {
            entries
                .iter()
                .map(|entry| entry.get_bloc_duration(&date))
                .sum()
        }
    };

    if arg_matches.get_flag("explicit") {
        print_arrow(stps(sum).bold(), "green");
    } else {
        println!("{}", stps(sum).bold());
    }
    Ok(())
}
