mod algorithms;
mod animation;
mod cli;
mod config;
mod data;
mod dirs;
mod utils;

use anyhow::Context;
use directories::ProjectDirs;

use cli::{
    process_add_subcommand, process_backup_subcommand, process_graph_subcommand,
    process_list_subcommand, process_prune_subcommand, process_reindex_subcommand,
    process_remove_subcommand, process_start_subcommand, process_sum_subcommand,
    process_today_subcommand, process_unindex_subcommand, process_update_subcommand,
    ProcessOutput as PO,
};
use data::{internal::Entries, syrtime::syrdate::SyrDate};
use tracing::{debug, info, trace, warn};
use tracing_subscriber::{fmt::time::Uptime, EnvFilter};

fn main() -> anyhow::Result<()> {
    // Not great but not catastrophic either, this won't stop the application
    if let Err(e) = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info")))
        .with_timer(Uptime::default())
        .compact()
        .try_init()
    {
        eprintln!("Failed to initialize tracing: '{e}'")
    }

    debug!("Gathering and checking required directories");
    let dirs =
        ProjectDirs::from("", "", "syracuse").context("Failed to get project directories")?;

    let dirs_check = || -> anyhow::Result<()> {
        let conf_dir = dirs.config_dir();
        if !conf_dir.exists() {
            warn!("Missing config directory");
            std::fs::create_dir_all(conf_dir).context(format!(
                "Failed to create the config directory at: '{}'",
                conf_dir.display()
            ))?;
            info!("Created the config directory at: '{}'", conf_dir.display())
        }
        let data_dir = dirs.data_dir();
        if !data_dir.exists() {
            warn!("Missing data directory");
            std::fs::create_dir_all(data_dir).context(format!(
                "Failed to create the data directory at: '{}'",
                data_dir.display(),
            ))?;
            info!("Created the data directory at: '{}'", data_dir.display())
        }
        Ok(())
    };
    dirs_check()?;

    // this should never fail, unwrapping is fine
    debug!("Locking the directories and the program's configuration");
    config::CONFIG
        .set(config::Config::load(
            &dirs.config_dir().join("syracuse.conf"),
        ))
        .unwrap();
    dirs::DIRS.set(dirs).unwrap();

    // not the biggest fan of this to be honest, I'd rather have access to the warning
    let datetime = jiff::Zoned::now();
    debug!("Acquired local datetime: {}", datetime);
    let datetime = datetime.datetime();

    let time = datetime.time();
    let date: SyrDate = {
        if time.hour() < config::Config::get().night_owl_hour_extension {
            datetime.date().yesterday()?.into()
        } else {
            datetime.date().into()
        }
    };

    let entries = Entries::load()?;

    let command = cli::cli();
    let arg_matches = command.get_matches();
    let _ = config::VERBOSE
        .set(arg_matches.get_flag("verbose"))
        .map_err(|err| warn!("failed to enable verbose output: '{err}'"));
    println!();

    match process_add_subcommand(&arg_matches, &entries)? {
        PO::Continue(_) => (),
        PO::Terminate => return Ok(()),
    }

    match process_list_subcommand(&arg_matches, &entries)? {
        PO::Continue(_) => (),
        PO::Terminate => return Ok(()),
    }

    match process_remove_subcommand(&arg_matches, &entries)? {
        PO::Continue(_) => (),
        PO::Terminate => return Ok(()),
    }

    match process_start_subcommand(&arg_matches, &entries, &date, &time)? {
        PO::Continue(_) => (),
        PO::Terminate => return Ok(()),
    }

    match process_update_subcommand(&arg_matches, &entries, &date)? {
        PO::Continue(_) => (),
        PO::Terminate => return Ok(()),
    }

    match process_today_subcommand(&arg_matches, &entries, &date)? {
        PO::Continue(_) => (),
        PO::Terminate => return Ok(()),
    }

    match process_backup_subcommand(&arg_matches, &entries, &datetime)? {
        PO::Continue(_) => (),
        PO::Terminate => return Ok(()),
    }

    match process_unindex_subcommand(&arg_matches, &entries)? {
        PO::Continue(_) => (),
        PO::Terminate => return Ok(()),
    }

    match process_reindex_subcommand(&arg_matches, &entries)? {
        PO::Continue(_) => (),
        PO::Terminate => return Ok(()),
    }

    match process_sum_subcommand(&arg_matches, &entries, &date)? {
        PO::Continue(_) => (),
        PO::Terminate => return Ok(()),
    }

    let entries = match process_prune_subcommand(&arg_matches, entries)? {
        PO::Continue(entries) => entries.unwrap(),
        PO::Terminate => return Ok(()),
    };

    match process_graph_subcommand(&arg_matches, entries, &date)? {
        PO::Continue(_) => (),
        PO::Terminate => return Ok(()),
    }

    Ok(())
}
