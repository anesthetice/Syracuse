mod algorithms;
mod animation;
mod cli;
mod config;
mod data;
mod dirs;
mod utils;

use anyhow::Context;
use data::{internal::Entries, syrtime::syrdate::SyrDate};
use directories::ProjectDirs;

fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .parse_default_env()
        .format_timestamp(None)
        .init();

    log::debug!("Gathering and checking required directories");
    let dirs =
        ProjectDirs::from("", "", "syracuse").context("Failed to get project directories")?;

    let dirs_check = || -> anyhow::Result<()> {
        let conf_dir = dirs.config_dir();
        if !conf_dir.exists() {
            log::warn!("Missing config directory");
            std::fs::create_dir_all(conf_dir).context(format!(
                "Failed to create the config directory at: '{}'",
                conf_dir.display()
            ))?;
            log::info!("Created the config directory at: '{}'", conf_dir.display())
        }
        let data_dir = dirs.data_dir();
        if !data_dir.exists() {
            log::warn!("Missing data directory");
            std::fs::create_dir_all(data_dir).context(format!(
                "Failed to create the data directory at: '{}'",
                data_dir.display(),
            ))?;
            log::info!("Created the data directory at: '{}'", data_dir.display())
        }
        Ok(())
    };
    dirs_check()?;

    // this should never fail, unwrapping is fine
    log::debug!("Locking settings...");
    config::CONFIG
        .set(config::Config::load(
            &dirs.config_dir().join("syracuse.conf"),
        ))
        .unwrap();
    dirs::DIRS.set(dirs).unwrap();

    // not the biggest fan of this to be honest, I'd rather have access to the warning
    let datetime = jiff::Zoned::now();
    log::debug!("Local datetime: {}", datetime);
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

    cli::cli(entries, date, datetime)?;

    Ok(())
}
