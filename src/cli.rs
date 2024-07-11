use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

use anyhow::Context;
use clap::{command, value_parser, Arg, ArgAction, ArgGroup, ArgMatches, Command};
use crossterm::{event, style::Stylize};
use time::{OffsetDateTime, Time};

use crate::{
    animation, config,
    data::{
        graph,
        internal::{Entries, Entry, IndexOptions},
        syrtime::{ns_to_pretty_string, SyrDate},
    },
    error, info,
    utils::{enter_clean_input_mode, exit_clean_input_mode},
    warn,
};

pub fn cli() -> clap::Command {
    let add_subcommand = Command::new("add")
        .alias("new")
        .about("Add a new entry to syracuse")
        .long_about("This subcommand is used to add a new entry to syracuse, entries are case-insensitive and can have aliases\naliases: 'new'")
        .arg(Arg::new("entry")
                .index(1)
                .num_args(1..10)
                .required(true)
                .help("entry to add")
                .long_help("entry to add\ne.g. 'add math-201 analysis' will add an entry titled 'MATH-201' with the alias 'ANALYSIS'")
                .action(ArgAction::Set)
            );

    let list_subcommand = Command::new("list")
        .alias("ls")
        .about("List out all entries")
        .long_about("This subcommand is used to list out all entries stored\naliases: 'ls'")
        .arg(
            Arg::new("full")
                .short('f')
                .short_alias('a')
                .long("full")
                .alias("all")
                .num_args(0)
                .required(false)
                .help("prints out the data associated with each entry as well")
                .action(ArgAction::SetTrue),
        );

    let remove_subcommand = Command::new("remove")
        .aliases(["rm", "delete", "del"])
        .about("Remove a single entry")
        .long_about("This subcommand is used to remove a single entry at a time from syracuse.json\naliases: 'rm', 'delete', 'del'")
        .arg(
            Arg::new("entry")
                .index(1)
                .required(true)
                .help("entry to remove")
                .action(ArgAction::Set),
        );

    let start_subcommand = Command::new("start")
        .aliases(["s", "r", "run", "go", "launch", "begin"])
        .about("Start the daily stopwatch for an entry")
        .long_about("This subcommand is used to start counting up the time spent today on the given entry, will progressively update syracuse.json\naliases: 's', 'r', 'run', 'go', 'launch', 'begin'")
        .arg(
            Arg::new("entry")
                .index(1)
                .required(true)
                .help("entry to start")
                .action(ArgAction::Set),
        );

    let update_subcommand = Command::new("update")
        .about("Manually update the time of an entry")
        .long_about("This subcommand is used to manually increase or decrease the time associated with an entry on a given day")
        .arg(
            Arg::new("entry")
                .index(1)
                .required(true)
                .help("entry to update")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("operation")
                .index(2)
                .required(true)
                .help("add or sub")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("date")
                .required(false)
                .help("the targeted date")
                .long_help("the targeted date, defaults to today")
                .short('d')
                .long("date")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("hour")
                .required(false)
                .help("the number of hours to add or subtract")
                .short('t')
                .long("hour")
                .value_parser(value_parser!(f64))
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("minute")
                .required(false)
                .help("the number of minutes to add or subtract")
                .short('m')
                .long("minute")
                .value_parser(value_parser!(f64))
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("second")
                .required(false)
                .help("the number of seconds to add or subtract")
                .short('s')
                .long("second")
                .value_parser(value_parser!(f64))
                .action(ArgAction::Set),
        );

    let today_subcommand = Command::new("today")
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
        );

    let backup_subcommand = Command::new("backup")
        .about("Backup entries")
        .long_about("This subcommand is used to backup all entries to a directory specified in the configuration file or directly provided by the user")
        .arg(
            Arg::new("path")
                .help("specified path")
                .index(1)
                .action(ArgAction::Set)
        );

    let unindex_subcommand = Command::new("unindex")
        .about("Unindexes a specified entry")
        .long_about("This subcommand is used to unindex a specified entry, meaning it will not appear within the choice pool for other command")
        .arg(
            Arg::new("entry")
                .index(1)
                .required(true)
                .help("entry to unindex")
                .action(ArgAction::Set),
        );

    let reindex_subcommand = Command::new("reindex")
        .about("Reindexes a specified entry")
        .long_about(
            "This subcommand is used to reindex a specified entry that was previously unindexed",
        )
        .arg(
            Arg::new("entry")
                .index(1)
                .required(true)
                .help("entry to unindex")
                .action(ArgAction::Set),
        );

    let sum_subcommand = Command::new("sum")
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
        );

    let prune_subcommand = Command::new("prune")
        .about("Keeps only the blocs younger than a certain date old")
        .long_about(
            "This subcommand is used to remove blocs of time that are older than the provided date",
        )
        .arg(
            Arg::new("date")
                .help("cutoff date for pruning")
                .index(1)
                .required(true)
                .action(ArgAction::Set),
        );

    let graph_subcommand = Command::new("graph")
        .about("Creates a graph")
        .long_about("This subcommand is used to graph the entries within a specified time frame")
        .arg(
            Arg::new("days-back")
                .help("number of days back included")
                .short('d')
                .long("days")
                .alias("days-back")
                .value_parser(value_parser!(usize))
                .action(ArgAction::Set)
                .group("logic-group"),
        )
        .arg(
            Arg::new("start-date")
                .help("start date")
                .short('s')
                .long("start")
                .alias("start-date")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("end-date")
                .help("end date")
                .short('e')
                .long("end")
                .alias("end-date")
                .action(ArgAction::Set),
        )
        .group(ArgGroup::new("logic-group").conflicts_with("start-date"));

    command!()
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .alias("debug")
                .short('v')
                .required(false)
                .global(true)
                .action(ArgAction::SetTrue),
        )
        .subcommands([
            add_subcommand,
            list_subcommand,
            remove_subcommand,
            start_subcommand,
            update_subcommand,
            today_subcommand,
            backup_subcommand,
            unindex_subcommand,
            reindex_subcommand,
            sum_subcommand,
            prune_subcommand,
            graph_subcommand,
        ])
}

// might not be the prettiest way of doing things
// but it's not so bad, and it lets me keep main.rs pretty clean
pub enum ProcessOutput {
    Continue(Option<Entries>),
    Terminate,
}

use ProcessOutput as PO;
pub fn process_add_subcommand(
    arg_matches: &ArgMatches,
    entries: &Entries,
) -> anyhow::Result<ProcessOutput> {
    let Some(arg_matches) = arg_matches.subcommand_matches("add") else {
        return Ok(PO::Continue(None));
    };
    let Some(entry_match) = arg_matches.get_many::<String>("entry") else {
        Err(error::Error {}).context("failed to parse entry as string")?
    };
    let mut names: Vec<String> = entry_match.map(|s| s.to_uppercase()).collect();

    let separator_characters = config::Config::get().entry_file_name_separtor.as_str();
    for name in names.iter() {
        if name.contains(separator_characters) {
            Err(error::Error{})
                    .with_context(|| format!("failed to add new entry, the name and or aliases provided conflict with the separator characters: '{}'", separator_characters))?
        }
    }

    for entry in entries.iter() {
        for name in names.iter() {
            if !entry.check_new_entry_name_validity(name) {
                Err(error::Error{})
                    .with_context(|| format!("failed to add new entry, the name and or aliases provided conflict with an existing entry: '{}'", entry))?
            }
        }
    }

    Entry::new(names.remove(0), names).save()?;
    Ok(PO::Terminate)
}

pub fn process_list_subcommand(
    arg_matches: &ArgMatches,
    entries: &Entries,
) -> anyhow::Result<ProcessOutput> {
    let Some(arg_matches) = arg_matches.subcommand_matches("list") else {
        return Ok(PO::Continue(None));
    };
    if arg_matches.get_flag("full") {
        for entry in entries.iter() {
            println!("{:?}\n", entry)
        }
    } else {
        for entry in entries.iter() {
            println!("{}\n", entry)
        }
    }
    Ok(PO::Terminate)
}

pub fn process_remove_subcommand(
    arg_matches: &ArgMatches,
    entries: &Entries,
) -> anyhow::Result<ProcessOutput> {
    let Some(arg_matches) = arg_matches.subcommand_matches("remove") else {
        return Ok(PO::Continue(None));
    };
    let Some(entry_match) = arg_matches.get_one::<String>("entry") else {
        Err(error::Error {}).context("failed to parse entry as string")?
    };

    let Some(entry) = entries.choose(entry_match.to_uppercase().as_str(), IndexOptions::Indexable)
    else {
        return Ok(PO::Terminate);
    };
    entry.delete()?;
    Ok(PO::Terminate)
}

pub fn process_start_subcommand(
    arg_matches: &ArgMatches,
    entries: &Entries,
    today: &SyrDate,
    time: &Time,
) -> anyhow::Result<ProcessOutput> {
    let Some(arg_matches) = arg_matches.subcommand_matches("start") else {
        return Ok(PO::Continue(None));
    };
    let Some(entry_match) = arg_matches.get_one::<String>("entry") else {
        Err(error::Error {}).context("failed to parse entry as string")?
    };
    let Some(mut entry) =
        entries.choose(entry_match.to_uppercase().as_str(), IndexOptions::Indexable)
    else {
        return Ok(PO::Terminate);
    };
    // start of initialization
    let mut file_save_error_counter: u8 = 0;
    let frame_period = config::Config::get().frame_period;
    let mut animation =
        animation::Animation::construct(config::Config::get().animation.clone(), 12, 12);
    let start = Instant::now();
    let mut instant = start;
    let mut autosave_instant = start;
    let autosave_perdiod = Duration::from_secs(config::Config::get().autosave_period as u64);
    let mut stdout = std::io::stdout();
    enter_clean_input_mode();
    if config::Config::get().stopwatch_explicit {
        let (h, m, s) = time.as_hms();
        println!("stopwatch started at: {:0>2}:{:0>2}:{:0>2}\n", h, m, s)
    }
    // end of initialization
    loop {
        animation.step(
            &mut stdout,
            &ns_to_pretty_string(instant.duration_since(start).as_nanos()),
        );
        if event::poll(std::time::Duration::from_millis(frame_period))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press
                    && (key.code == event::KeyCode::Char('q')
                        || key.code == event::KeyCode::Char('Q')
                        || key.code == event::KeyCode::Enter)
                {
                    break;
                }
            }
        }
        if instant.duration_since(autosave_instant) > autosave_perdiod {
            if let Err(error) = entry.save() {
                file_save_error_counter += 1;
                if file_save_error_counter > 2 {
                    warn!("maximum number of failed autosaves reached, exiting...");
                    return Err(error);
                } else {
                    warn!("failed to autosave progress: '{}'", error);
                }
            }
            autosave_instant = instant;
        }
        let new_instant = Instant::now();
        entry.increase_bloc_duration(today, new_instant.duration_since(instant).as_nanos());
        instant = new_instant;
    }
    exit_clean_input_mode();
    println!();
    entry.save().context("failed to save entry progress")?;
    Ok(PO::Terminate)
}

pub fn process_update_subcommand(
    arg_matches: &ArgMatches,
    entries: &Entries,
    today: &SyrDate,
) -> anyhow::Result<ProcessOutput> {
    let Some(arg_matches) = arg_matches.subcommand_matches("update") else {
        return Ok(PO::Continue(None));
    };
    let date = match arg_matches.get_one::<String>("date") {
        Some(string) => SyrDate::try_from(string.as_str())?,
        None => *today,
    };
    let Some(operation) = arg_matches.get_one::<String>("operation") else {
        Err(error::Error {}).context("failed to parse operation as string")?
    };
    let Some(entry_match) = arg_matches.get_one::<String>("entry") else {
        Err(error::Error {}).context("failed to parse entry as string")?
    };
    let Some(mut entry) =
        entries.choose(entry_match.to_uppercase().as_str(), IndexOptions::Indexable)
    else {
        return Ok(PO::Terminate);
    };
    let hour_diff: f64 = *arg_matches.get_one::<f64>("hour").unwrap_or(&0.0);
    let minute_diff: f64 = *arg_matches.get_one::<f64>("minute").unwrap_or(&0.0);
    let second_diff: f64 = *arg_matches.get_one::<f64>("second").unwrap_or(&0.0);
    let total_diff: u128 = (hour_diff * 3_600_000_000_000_f64
        + minute_diff * 60_000_000_000_f64
        + second_diff * 1_000_000_000_f64) as u128;

    if ["add", "plus", "incr", "increase"]
        .iter()
        .any(|s| *s == operation)
    {
        let tmp = ns_to_pretty_string(entry.get_block_duration(&date));
        entry.increase_bloc_duration(&date, total_diff);
        entry.save()?;
        println!(
            "{}  :  {} {} {}",
            &date,
            &tmp,
            "――>".green(),
            ns_to_pretty_string(entry.get_block_duration(&date))
        )
    } else if ["sub", "rm", "rem", "remove", "minus", "decr", "decrease"]
        .iter()
        .any(|s| *s == operation)
    {
        let tmp = ns_to_pretty_string(entry.get_block_duration(&date));
        entry.decrease_bloc_duration(&date, total_diff);
        entry.save()?;
        println!(
            "{}  :  {} {} {}",
            &date,
            &tmp,
            "――>".red(),
            ns_to_pretty_string(entry.get_block_duration(&date))
        )
    } else {
        warn!("unknown operation: '{}'", operation);
    }
    Ok(PO::Terminate)
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

pub fn process_backup_subcommand(
    arg_matches: &ArgMatches,
    entries: &Entries,
    today_datetime: &OffsetDateTime,
) -> anyhow::Result<ProcessOutput> {
    let Some(arg_matches) = arg_matches.subcommand_matches("backup") else {
        return Ok(PO::Continue(None));
    };
    let folder = format!(
        "{:0>4}_{:0>2}_{:0>2}-{:0>2}_{:0>2}_{:0>2}/",
        today_datetime.year(),
        today_datetime.month() as u8,
        today_datetime.day(),
        today_datetime.hour(),
        today_datetime.minute(),
        today_datetime.second(),
    );

    let path = match arg_matches.get_one::<String>("path") {
        Some(string) => PathBuf::from(string),
        None => PathBuf::from(config::Config::get().backup_path.as_str()),
    }
    .join(folder);

    if let Err(error) = std::fs::create_dir(&path) {
        if error.kind() != std::io::ErrorKind::AlreadyExists {
            warn!(
                "failed to create the following directory: '{:?}', caused by: '{}'",
                &path, error
            );
            return Ok(PO::Terminate);
        } else {
            info!("directory already exists, this is not feasible");
        }
    }
    println!("backing up to: '{}'", &path.display());

    entries.backup(path);
    Ok(PO::Terminate)
}

pub fn process_unindex_subcommand(
    arg_matches: &ArgMatches,
    entries: &Entries,
) -> anyhow::Result<ProcessOutput> {
    let Some(arg_matches) = arg_matches.subcommand_matches("unindex") else {
        return Ok(PO::Continue(None));
    };

    let Some(entry_match) = arg_matches.get_one::<String>("entry") else {
        Err(error::Error {}).context("failed to parse entry as string")?
    };
    let Some(mut entry) =
        entries.choose(entry_match.to_uppercase().as_str(), IndexOptions::Indexable)
    else {
        return Ok(PO::Terminate);
    };

    entry.inverse_indexability()?;
    info!("unindexed '{}'", entry.get_name());

    Ok(PO::Terminate)
}

pub fn process_reindex_subcommand(
    arg_matches: &ArgMatches,
    entries: &Entries,
) -> anyhow::Result<ProcessOutput> {
    let Some(arg_matches) = arg_matches.subcommand_matches("reindex") else {
        return Ok(PO::Continue(None));
    };

    let Some(entry_match) = arg_matches.get_one::<String>("entry") else {
        Err(error::Error {}).context("failed to parse entry as string")?
    };
    let Some(mut entry) = entries.choose(
        entry_match.to_uppercase().as_str(),
        IndexOptions::Unindexable,
    ) else {
        return Ok(PO::Terminate);
    };

    entry.inverse_indexability()?;
    info!("reindexed '{}'", entry.get_name());

    Ok(PO::Terminate)
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

pub fn process_prune_subcommand(
    arg_matches: &ArgMatches,
    mut entries: Entries,
) -> anyhow::Result<ProcessOutput> {
    let Some(arg_matches) = arg_matches.subcommand_matches("prune") else {
        return Ok(PO::Continue(Some(entries)));
    };
    let Some(cutoff_date) = arg_matches.get_one::<String>("date") else {
        Err(error::Error {}).context("failed to parse date as string")?
    };

    let cutoff_date = SyrDate::try_from(cutoff_date.as_str())?;
    let mut sum: usize = 0;
    for entry in entries.iter_mut() {
        sum += entry.prune(&cutoff_date)?;
    }
    println!(
        "{}",
        format!("{} {} pruned", sum, if sum == 1 { "bloc" } else { "blocs" }).bold()
    );

    Ok(PO::Terminate)
}

pub fn process_graph_subcommand(
    arg_matches: &ArgMatches,
    entries: Entries,
    today: &SyrDate,
) -> anyhow::Result<ProcessOutput> {
    let Some(arg_matches) = arg_matches.subcommand_matches("graph") else {
        return Ok(PO::Continue(Some(entries)));
    };

    // days-back + specified end-date or not
    if let Some(num) = arg_matches.get_one::<usize>("days-back") {
        // starts off as end date but will become the start date therefore it's already called start date
        let mut start_date = match arg_matches.get_one::<String>("end-date") {
            Some(string) => SyrDate::try_from(string.as_str()).unwrap_or(*today),
            None => *today,
        };
        for _ in 0..*num {
            start_date = start_date
                .previous_day()
                .ok_or(crate::error::Error {})
                .context("invalid number of days back, this is highly unlikely")?
                .into();
        }
        graph::graph(entries, start_date, *today)?;
        Ok(PO::Terminate)
    }
    // start-date + specified end-date or not
    else {
        let Some(start_date) = arg_matches.get_one::<String>("start-date") else {
            Err(error::Error {}).context("failed to parse starting date as string")?
        };
        let start_date = SyrDate::try_from(start_date.as_str())?;

        let end_date = match arg_matches.get_one::<String>("end-date") {
            Some(string) => SyrDate::try_from(string.as_str()).unwrap_or(*today),
            None => *today,
        };

        if start_date > end_date {
            Err(error::Error {}).context("starting date is larger than ending date")?
        }

        crate::data::graph::graph(entries, start_date, end_date)?;
        Ok(PO::Terminate)
    }
}
