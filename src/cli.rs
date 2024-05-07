use std::time::{Duration, Instant};

use anyhow::Context;
use clap::{command, Arg, ArgAction, ArgMatches, Command};
use crossterm::{event, style::Stylize};

use crate::{
    animation,
    config,
    data::{internal::{Entries, Entry}, syrtime::{ns_to_pretty_string, SyrDate}},
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
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("minute")
                .required(false)
                .help("the number of minutes to add or subtract")
                .short('m')
                .long("minute")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("second")
                .required(false)
                .help("the number of seconds to add or subtract")
                .short('s')
                .long("second")
                .action(ArgAction::Set),
        );

    let graph_subcommand = Command::new("graph")
        .about("Creates a graph")
        .long_about("This subcommand is used to graph the entries within a specified time frame")
        .arg(
            Arg::new("all")
                .help("graphs all entries")
                .exclusive(true)
                .short('a')
                .short('f')
                .long("all")
                .alias("full")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("single")
                .help("graphs a single specified entry")
                .exclusive(true)
                .short('s')
                .long("single")
                .required(false)
                .action(ArgAction::Set),
        );

    command!()
        .subcommand(add_subcommand)
        .subcommand(list_subcommand)
        .subcommand(remove_subcommand)
        .subcommand(start_subcommand)
        .subcommand(update_subcommand)
        .subcommand(graph_subcommand)
}

pub enum ProcessOutput {
    Continue,
    Terminate, 
}

use ProcessOutput as PO;
pub fn process_add_subcommand(arg_matches: &ArgMatches, entries: &Entries) -> anyhow::Result<ProcessOutput> {
    let Some(arg_matches) = arg_matches.subcommand_matches("add") else {
        return Ok(PO::Continue)
    };
    let Some(entry_match) = arg_matches.get_many::<String>("entry") else {
        info!("invalid entries");
        return Ok(PO::Terminate)
    };
    let mut names: Vec<String> = entry_match.map(|s| s.to_uppercase()).collect();

    for entry in entries.iter() {
        let filestem = entry.get_filestem();
        for name in names.iter() {
            if filestem.contains(name) {
                Err(error::Error{})
                    .with_context(|| format!("failed to add new entry, the name and alisases provided conflict with an existing entry or the separator characters, {}", filestem))?
            }
        }
    }

    Entry::new(names.remove(0), names).save_to_file()?;
    Ok(PO::Terminate)       
}

pub fn process_list_subcommand(arg_matches: &ArgMatches, entries: &Entries) -> anyhow::Result<ProcessOutput> {
    let Some(arg_matches) = arg_matches.subcommand_matches("list") else {
        return Ok(PO::Continue)
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
        return Ok(PO::Continue)
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
        return Ok(PO::Continue)
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
                    && (key.code == event::KeyCode::Char('q') || key.code == event::KeyCode::Enter)
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
        return Ok(PO::Continue)
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
    let hour_diff: f64 = match arg_matches.get_one::<String>("hour") {
        Some(val) => val.parse::<f64>().unwrap_or(0.0),
        None => 0.0,
    };
    let minute_diff: f64 = match arg_matches.get_one::<String>("minute") {
        Some(val) => val.parse::<f64>().unwrap_or(0.0),
        None => 0.0,
    };
    let second_diff: f64 = match arg_matches.get_one::<String>("second") {
        Some(val) => val.parse::<f64>().unwrap_or(0.0),
        None => 0.0,
    };
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

/*
pub fn process__subcommand(arg_matches: &ArgMatches, entries: &Entries) -> anyhow::Result<ProcessOutput> {
    let Some(arg_matches) = arg_matches.subcommand_matches("") else {
        return Ok(PO::Continue)
    };

    
    Ok(PO::Terminate)
}
*/