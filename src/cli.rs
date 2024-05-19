use std::time::{Duration, Instant};

use anyhow::Context;
use clap::{command, value_parser, Arg, ArgAction, ArgMatches, Command};
use crossterm::{event, style::Stylize};

use crate::{
    animation,
    config,
    data::{graph, internal::{Entries, Entry}, syrtime::{ns_to_pretty_string, SyrDate}},
    error, info, utils::{enter_clean_input_mode, exit_clean_input_mode}, warn,
};

pub fn cli() -> clap::Command {
    let add_subcommand = Command::new("add")
        .alias("new")
        .about("Add a new entry to syracuse\naliases: 'new'")
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
        .about("Lists out all entries")
        .long_about("This subcommand is used to list out all entries stored in syracuse.json")
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
        .aliases(["delete", "del"])
        .about("Removes a single entry\naliases: 'delete', 'del'")
        .long_about("This subcommand is used to remove a single entry at a time from syracuse.json\naliases: 'delete', 'del'")
        .arg(
            Arg::new("entry")
                .index(1)
                .required(true)
                .help("entry to remove")
                .action(ArgAction::Set),
        );

    let start_subcommand = Command::new("start")
        .aliases(["s", "r", "run", "go", "launch", "begin"])
        .about("Starts the daily stopwatch for the given entry")
        .long_about("This subcommand is used to start counting up the time spent today on the given entry, will progressively update syracuse.json\naliases: 's', 'r', 'run', 'go', 'launch', 'begin'")
        .arg(
            Arg::new("entry")
                .index(1)
                .required(true)
                .help("entry to start")
                .action(ArgAction::Set),
        );

    let update_subcommand = Command::new("update")
        .about("Manually updates the time of an entry")
        .long_about("This subcommand is used to manually increase or decrease the time associated with an entry on a given day")
        .arg(
            Arg::new("operation")
                .index(1)
                .required(true)
                .help("add or sub")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("entry")
                .index(2)
                .required(true)
                .help("entry to update")
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
        .about("Displays time tracked today")
        .long_about("This subcommand is used to display the sum of the time tracked by every single entry for today");

    let prune_subcommand = Command::new("prune")
        .about("Removes old blocs from entries")
        .long_about("This subcommand is used to remove blocs of time that are older than the provided date")
        .arg(
            Arg::new("date")
                .help("cutoff date for pruning")
                .index(1)
                .required(true)
                .action(ArgAction::Set)
        );

    let graph_subcommand = Command::new("graph")
        .about("Creates a graph")
        .long_about("This subcommand is used to graph the entries within a specified time frame")
        .arg(
            Arg::new("days")
                .help("number of days back graphed")
                .short('d')
                .long("days")
                .value_parser(value_parser!(usize))
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("start")
                .help("start date")
                .short('s')
                .long("start")
                .action(ArgAction::Set)
        )
        .arg(
            Arg::new("end")
                .help("end date")
                .short('e')
                .long("end")
                .action(ArgAction::Set)
        );

    command!().subcommands([
        add_subcommand,
        list_subcommand,
        remove_subcommand,
        start_subcommand,
        update_subcommand,
        today_subcommand,
        prune_subcommand,
        graph_subcommand
    ])
}

pub enum ProcessOutput {
    Continue(Option<Entries>),
    Terminate, 
}

use ProcessOutput as PO;
pub fn process_add_subcommand(arg_matches: &ArgMatches, entries: &Entries) -> anyhow::Result<ProcessOutput> {
    let Some(arg_matches) = arg_matches.subcommand_matches("add") else {
        return Ok(PO::Continue(None))
    };
    let Some(entry_match) = arg_matches.get_many::<String>("entry") else {
        info!("invalid entries");
        return Ok(PO::Terminate)
    };
    let mut names: Vec<String> = entry_match.map(|s| s.to_uppercase()).collect();

    let separator_characters = config::Config::get().entry_file_name_separtor.as_str();
    for name in names.iter() {
        if name.contains(separator_characters) {
            Err(error::Error{})
                    .with_context(|| format!("failed to add new entry, the name and or aliases provided conflict with the separator characters '{}'", separator_characters))?
        }
    }

    for entry in entries.iter() {
        for name in names.iter() {
            if !entry.check_new_entry_name_validity(name) {
                Err(error::Error{})
                    .with_context(|| format!("failed to add new entry, the name and or aliases provided conflict with an existing entry, {}", entry))?
            }
        }
    }

    Entry::new(names.remove(0), names).save_to_file()?;
    Ok(PO::Terminate)       
}

pub fn process_list_subcommand(arg_matches: &ArgMatches, entries: &Entries) -> anyhow::Result<ProcessOutput> {
    let Some(arg_matches) = arg_matches.subcommand_matches("list") else {
        return Ok(PO::Continue(None))
    };
    if arg_matches.get_flag("full") {
        for entry in entries.iter() {println!("{:?}\n", entry)}
    } else {
        for entry in entries.iter() {println!("{}\n", entry)}
    }
    Ok(PO::Terminate)
}

pub fn process_remove_subcommand(arg_matches: &ArgMatches, entries: &Entries) -> anyhow::Result<ProcessOutput> {
    let Some(arg_matches) = arg_matches.subcommand_matches("remove") else {
        return Ok(PO::Continue(None))
    };
    let Some(entry_match) = arg_matches.get_one::<String>("entry") else {
        return Ok(PO::Terminate)
    };

    if let Some(entry) = entries.choose(entry_match.to_uppercase().as_str()) {
        entry.delete()?;
        Ok(PO::Terminate)
    } else {
        Ok(PO::Terminate)
    }
}

pub fn process_start_subcommand(arg_matches: &ArgMatches, entries: &Entries, today: &SyrDate) -> anyhow::Result<ProcessOutput> {
    let Some(arg_matches) = arg_matches.subcommand_matches("start") else {
        return Ok(PO::Continue(None))
    };
    let Some(entry_match) = arg_matches.get_one::<String>("entry") else {
        return Ok(PO::Terminate)
    };
    let Some(mut entry) = entries.choose(entry_match.to_uppercase().as_str()) else {
        return Ok(PO::Terminate)
    };
    // start of initialization
    let mut file_save_error_counter: u8 = 0;
    let frame_period = config::Config::get().frame_period;
    let mut animation = animation::Animation::construct(
        config::Config::get().animation.clone(),
        12,
        12
    );
    let start = Instant::now();
    let mut instant = start;
    let mut autosave_instant = start;
    let autosave_perdiod = Duration::from_secs(config::Config::get().autosave_period as u64);
    let mut stdout = std::io::stdout();
    println!();
    enter_clean_input_mode();
    // end of initialization
    loop {
        animation.step(
            &mut stdout,
            &ns_to_pretty_string(instant.duration_since(start).as_nanos())
        );
        if event::poll(std::time::Duration::from_millis(frame_period))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press
                    && (key.code == event::KeyCode::Char('q') || key.code == event::KeyCode::Char('Q') || key.code == event::KeyCode::Enter)
                {break}
            }
        }
        if instant.duration_since(autosave_instant) > autosave_perdiod {
            if let Err(error) = entry.save_to_file() {
                file_save_error_counter += 1;
                if file_save_error_counter > 2 {
                    warn!("maximum number of failed autosaves reached, exiting...");
                    return Err(error);
                }
                else {
                    warn!("failed to autosave progress, {}", error);
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
    entry.save_to_file().context("failed to save entry progress")?;
    Ok(PO::Terminate)
}

pub fn process_update_subcommand(arg_matches: &ArgMatches, entries: &Entries, today: &SyrDate) -> anyhow::Result<ProcessOutput> {
    let Some(arg_matches) = arg_matches.subcommand_matches("update") else {
        return Ok(PO::Continue(None))
    };
    let date = match arg_matches.get_one::<String>("date") {
        Some(string) => SyrDate::try_from(string.as_str())?,
        None => *today,
    };
    let Some(operation) = arg_matches.get_one::<String>("operation") else {
        return Ok(PO::Terminate)
    };
    let Some(entry_match) = arg_matches.get_one::<String>("entry") else {
        return Ok(PO::Terminate)
    };
    let Some(mut entry) = entries.choose(entry_match.to_uppercase().as_str()) else {
        return Ok(PO::Terminate)
    };
    let hour_diff: f64 = *arg_matches.get_one::<f64>("hour").unwrap_or(&0.0);
    let minute_diff: f64 = *arg_matches.get_one::<f64>("minute").unwrap_or(&0.0);
    let second_diff: f64 = *arg_matches.get_one::<f64>("second").unwrap_or(&0.0);
    let total_diff: u128 = (hour_diff * 3_600_000_000_000_f64 + minute_diff * 60_000_000_000_f64 + second_diff * 1_000_000_000_f64) as u128;

    if ["add", "plus", "incr", "increase"].iter().any(|s| *s == operation) {
        entry.increase_bloc_duration(&date, total_diff);
        entry.save_to_file()?;
    }
    else if ["sub", "rem", "remove", "minus", "decr", "decrease"].iter().any(|s| *s == operation) {
        entry.decrease_bloc_duration(&date, total_diff);
        entry.save_to_file()?;
    }
    else {
        info!("invalid operation, got : {}", operation);
    }
    
    Ok(PO::Terminate)
}

pub fn process_today_subcommand(arg_matches: &ArgMatches, entries: &Entries, today: &SyrDate) -> anyhow::Result<ProcessOutput> {
    let Some(_) = arg_matches.subcommand_matches("today") else {
        return Ok(PO::Continue(None))
    };

    let sum: u128 = entries.iter().map(|entry| entry.get_block_duration(today)).sum();
    println!("=> {}", ns_to_pretty_string(sum));

    Ok(PO::Terminate)
}

pub fn process_prune_subcommand(arg_matches: &ArgMatches, mut entries: Entries) -> anyhow::Result<ProcessOutput> {
    let Some(arg_matches) = arg_matches.subcommand_matches("prune") else {
        return Ok(PO::Continue(Some(entries)))
    };
    let Some(cutoff_date) = arg_matches.get_one::<String>("date") else {
        return Ok(PO::Terminate)
    };
    let cutoff_date = SyrDate::try_from(cutoff_date.as_str())?;
    for entry in entries.iter_mut() {
        entry.prune(&cutoff_date)?;
    }
    Ok(PO::Terminate)
}

pub fn process_graph_subcommand(arg_matches: &ArgMatches, entries: Entries, today: &SyrDate) -> anyhow::Result<ProcessOutput> {
    let Some(arg_matches) = arg_matches.subcommand_matches("graph") else {
        return Ok(PO::Continue(Some(entries)))
    };

    if let Some(num) = arg_matches.get_one::<usize>("days") {
        let mut start_date = *today;
        for _ in 0..*num {
            start_date = start_date
                .previous_day()
                .ok_or(crate::error::Error{})
                .context("invalid number of days back, this is highly unlikely")?
                .into();
        }
        graph::graph(entries, start_date, *today)?;
        return Ok(PO::Terminate);
    }

    let Some(start_date) = arg_matches.get_one::<String>("start") else {
        return Ok(PO::Terminate)
    };
    let start_date = SyrDate::try_from(start_date.as_str())?;

    let Some(end_date) = arg_matches.get_one::<String>("end") else {
        return Ok(PO::Terminate)
    };
    let end_date = SyrDate::try_from(end_date.as_str())?;

    crate::data::graph::graph(entries, start_date, end_date)?;
    Ok(PO::Terminate)
}

/*
pub fn process__subcommand(arg_matches: &ArgMatches, entries: &Entries) -> anyhow::Result<ProcessOutput> {
    let Some(arg_matches) = arg_matches.subcommand_matches("") else {
        return Ok(PO::Continue(None))
    };

    
    Ok(PO::Terminate)
}
*/