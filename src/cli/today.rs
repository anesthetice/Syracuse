use super::*;

pub(super) fn today_subcommand() -> Command {
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

pub fn process_today_subcommand(
    arg_matches: &ArgMatches,
    entries: &Entries,
    today: &SyrDate,
) -> anyhow::Result<ProcessOutput> {
    let Some(arg_matches) = arg_matches.subcommand_matches("today") else {
        return Ok(PO::Continue(None));
    };

    let date: SyrDate = {
        let mut date = **today;
        if arg_matches.get_flag("yesterday") {
            date = date
                .previous_day()
                .ok_or(error::Error {})
                .context("failed to get yesterday's date, this should not occur")?;
        } else if let Some(val) = arg_matches.get_one::<usize>("previous") {
            for _ in 0..*val {
                date = date
                    .previous_day()
                    .ok_or(error::Error {})
                    .context("failed to get the previous date, this should not occur")?
            }
        }
        date.into()
    };

    let sum = {
        if arg_matches.get_flag("explicit") {
            entries
                .iter()
                .map(|entry| {
                    let duration = entry.get_block_duration(&date);
                    // 15 seems reasonable, I could check the length of every entry's name and get a better estimation
                    // but even that would not be perfect, I would have to count the valid grapheme clusters which adds a lot of complexity
                    // to what I itend as simple padding
                    if duration != 0 {
                        println!(
                            "{:<15} :   {}",
                            entry.get_name(),
                            ns_to_pretty_string(duration)
                        )
                    }
                    duration
                })
                .sum()
        } else {
            entries
                .iter()
                .map(|entry| entry.get_block_duration(&date))
                .sum()
        }
    };

    if arg_matches.get_flag("explicit") {
        println!("\n{} {}", "――>".green(), ns_to_pretty_string(sum).bold());
    } else {
        println!("{}", ns_to_pretty_string(sum).bold());
    }

    Ok(PO::Terminate)
}
