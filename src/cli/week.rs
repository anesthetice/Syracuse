use color_eyre::owo_colors::OwoColorize;

use super::*;

pub(super) fn subcommand() -> Command {
    Command::new("week")
        .about("Display the time tracked this week")
        .long_about("This subcommand is used to display the sum of the time tracked by every single entry for the current week")
        .arg(
            Arg::new("weeks-back")
                .help("The number of weeks-back to check")
                .required(false)
                .short('w')
                .short_alias('p')
                .long("weeks-back")
                .value_parser(value_parser!(usize))
                .action(ArgAction::Set),
        )
}

pub fn process(arg_matches: &ArgMatches, entries: &Entries, today: &SyrDate) -> Result<()> {
    let syrspan: SyrSpan = {
        let date = if let Some(val) = arg_matches.get_one::<usize>("weeks-back") {
            let val = if !matches!(today.weekday(), Weekday::Monday) {
                -((val + 1) as i32)
            } else {
                -(*val as i32)
            };
            today.nth_weekday(val, Weekday::Monday)?
        } else {
            **today
        };

        let (start, end) = match date.weekday() {
            Weekday::Monday => (date, date.nth_weekday(1, Weekday::Sunday)?),
            Weekday::Sunday => (date.nth_weekday(-1, Weekday::Monday)?, date),
            _ => (date.nth_weekday(-1, Weekday::Monday)?, date.nth_weekday(1, Weekday::Sunday)?),
        };

        SyrSpan::from_start_and_end(start, end)
    };
    let mut total_weekly_duration: f64 = 0.0;
    for date in syrspan.into_iter() {
        let entries: Vec<&Entry> = entries.iter().filter(|entry| entry.get_bloc_duration(&date) != 0.0).collect();

        let pad = entries
            .iter()
            .map(|entry| entry.name.len() + entry.aliases.first().map(|alias| alias.len()).unwrap_or(0))
            .max()
            .unwrap_or(1_usize)
            + 2;

        let weekday = date.weekday().to_string();
        let dashes = format!("{:-<1$}", "", (pad + 3 + f64::S_STR_LENGTH).max(weekday.len() + 13));
        println!("{}\n{}", (weekday + " - " + date.to_string().as_str()).bold(), dashes.as_str().dim());

        let total_daily_duration: f64 = entries
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
        println!("{} {}\n", ARROWHEAD.dark_green(), total_daily_duration.s_str());
        total_weekly_duration += total_daily_duration;
    }
    println!("{} {}", ARROW.green(), total_weekly_duration.s_str().bold());
    Ok(())
}
