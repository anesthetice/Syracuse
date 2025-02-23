use super::*;

pub(super) fn subcommand() -> Command {
    Command::new("today")
        .about("Display the time tracked today")
        .long_about("This subcommand is used to display the sum of the time tracked by every single entry for today")
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
        } else if let Some(val) = arg_matches.get_one::<usize>("days-back") {
            date = date.saturating_sub(Span::new().days(i64::try_from(*val)?));
        }
        date.into()
    };

    let entries: Vec<&Entry> = entries.iter().filter(|entry| entry.get_bloc_duration(today) != 0.0).collect();

    let pad = entries
        .iter()
        .map(|entry| entry.name.len() + entry.aliases.first().map(|alias| alias.len()).unwrap_or(0))
        .max()
        .unwrap_or(1_usize)
        + 2;

    let weekday = date.weekday().to_string();
    let dashes = format!("{:-<1$}", "", (pad + 3 + f64::S_STR_LENGTH).max(weekday.len() + 13));

    println!("{}\n{}", (weekday + " - " + date.to_string().as_str()).bold(), dashes.as_str().dim());

    let total_duration: f64 = entries
        .iter()
        .map(|entry| {
            let duration = entry.get_bloc_duration(&date);
            let pad = if !entry.aliases.is_empty() {
                pad + 8 // .dim() adds 4 bytes to the start and the end of the string
            } else {
                pad
            };
            println!("{:<width$} : {}", entry.display_name_and_first_alias(), duration.s_str(), width = pad);
            duration
        })
        .sum();
    println!("{} {}", ARROW.green(), total_duration.s_str().bold());

    Ok(())
}
