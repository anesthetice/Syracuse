use super::*;

pub(super) fn subcommand() -> Command {
    Command::new("today")
        .about("Display the time tracked today")
        .long_about("This subcommand is used to display the sum of the time tracked by every single entry for today")
        .arg(
            Arg::new("yesterday")
                .required(false)
                .short('y')
                .long("yesterday")
                .action(ArgAction::SetTrue),
        )
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
            date = date.saturating_sub(i64::try_from(*val)?.days());
        }
        date.into()
    };

    type CompactOutput<'a> = (Vec<(&'a str, Option<&'a str>, f64)>, usize, f64);
    let (mut bones, pad, total_duration): CompactOutput = entries
        .iter()
        .filter_map(|entry| {
            let duration = entry.get_block_duration_opt(&date)?;
            let name: &str = entry.name.as_str();
            let alias: Option<&str> = entry.aliases.first().map(String::as_str);
            let padding = name.len() + alias.map(str::len).unwrap_or(0) + 2;
            Some((name, alias, duration, padding))
        })
        .fold(
            (Vec::new(), 0, 0.0),
            |(mut output, pad, total_duration), (name, alias, duration, pad_)| {
                output.push((name, alias, duration));
                (output, pad.max(pad_), total_duration + duration)
            },
        );

    bones.sort_by(|a, b| match config::Config::get().sort_option {
        SortOptions::NameAscending => a.0.cmp(b.0),
        SortOptions::NameDescending => b.0.cmp(a.0),
        SortOptions::DurationAscending => a.2.total_cmp(&b.2),
        SortOptions::DurationDescending => b.2.total_cmp(&a.2),
    });

    let weekday = date.weekday().to_string();
    let dashes = format!("{:-<1$}", "", usize::max(pad + 3 + f64::S_STR_LENGTH, weekday.len() + 13));
    println!(
        "{}\n{}",
        (weekday + " - " + date.to_string().as_str()).bold(),
        dashes.as_str().dim()
    );

    bones.into_iter().for_each(|(name, alias, duration)| match alias {
        Some(alias) => {
            let title: String = format!("{}; {}", name, alias.dim());
            println!("{:<width$} : {}", title, duration.s_str(), width = pad + 8);
        }
        None => {
            println!("{:<width$} : {}", name, duration.s_str(), width = pad);
        }
    });

    println!("{} {}", ARROW.green(), total_duration.s_str().bold());

    Ok(())
}
