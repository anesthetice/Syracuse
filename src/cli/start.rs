use super::*;

pub(super) fn start_subcommand() -> Command {
    Command::new("start")
        .aliases(["s", "r", "run", "go", "launch", "begin"])
        .about("Start the daily stopwatch for an entry")
        .long_about("This subcommand is used to start counting up the time spent today on the given entry, will progressively update syracuse.json\naliases: 's', 'r', 'run', 'go', 'launch', 'begin'")
        .arg(
            Arg::new("entry")
                .index(1)
                .required(true)
                .help("entry to start")
                .action(ArgAction::Set),
        )
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
        entries.choose(entry_match.to_uppercase().as_str(), IndexOptions::Indexed)
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
