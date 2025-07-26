use color_eyre::eyre;

use crate::app::App;

mod algorithms;
mod animation;
mod app;
mod data;
mod dirs;
mod utils;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let app = App::load()?;

    println!();
    //cli::cli(entries, date, datetime)?;

    Ok(())
}
