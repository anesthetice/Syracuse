mod algorithms;
mod animation;
mod cli;
mod config;
mod data;
mod dirs;
mod utils;

use color_eyre::{
    eyre::{eyre, Context, OptionExt},
    Result,
};
use data::{internal::Entries, syrtime::syrdate::SyrDate};
use directories::ProjectDirs;

fn main() -> Result<()> {
    let dirs =
        ProjectDirs::from("", "", "syracuse").ok_or_eyre("Failed to get project directories")?;

    let _conf_dir = dirs.config_dir();
    if !_conf_dir.exists() {
        std::fs::create_dir_all(_conf_dir).wrap_err_with(|| {
            format!(
                "Failed to create the config directory at: '{}'",
                _conf_dir.display()
            )
        })?;
    }
    let _data_dir = dirs.data_dir();
    if !_data_dir.exists() {
        std::fs::create_dir_all(_data_dir).context(format!(
            "Failed to create the data directory at: '{}'",
            _data_dir.display(),
        ))?;
    }

    config::CONFIG
        .set(config::Config::load(
            &dirs.config_dir().join("syracuse.conf"),
        ))
        .map_err(|_| eyre!("Failed to lock the configuration"))?;
    dirs::DIRS.set(dirs).unwrap();

    let datetime = jiff::Zoned::now();
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
