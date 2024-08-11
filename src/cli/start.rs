use super::*;

pub(super) fn subcommand() -> Command {
    Command::new("start")
        .aliases(["s", "r", "run", "go", "launch", "begin"])
        .about("Start the daily stopwatch for an entry")
        .long_about("This subcommand is used to start counting up the time spent today on the given entry, will progressively update the associated file\naliases: 's', 'r', 'run', 'go', 'launch', 'begin'")
        .arg(
            Arg::new("entry")
                .index(1)
                .required(true)
                .help("entry to start")
                .action(ArgAction::Set),
        )
}

pub fn process(arg_matches: &ArgMatches, entries: &Entries, today: &SyrDate) -> anyhow::Result<()> {
    let name = arg_matches
        .get_one::<String>("entry")
        .ok_or(anyhow!("Failed to parse entry to string"))?;

    let Some(mut entry) = entries.choose(&name.to_uppercase(), IndexOptions::Indexed) else {
        return Ok(());
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
    // end of initialization
    loop {
        animation.step(
            &mut stdout,
            &sec_to_pretty_string(instant.duration_since(start).as_secs_f64()),
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
                    error!("Maximum number of failed autosaves reached, exiting...");
                    return Err(error);
                } else {
                    warn!("Failed to autosave progress: '{}'", error);
                }
            }
            autosave_instant = instant;
        }
        let new_instant = Instant::now();
        entry.increase_bloc_duration(today, new_instant.duration_since(instant).as_secs_f64());
        instant = new_instant;
    }
    exit_clean_input_mode();
    println!();
    entry.save().context("Failed to save final entry progress")
}
