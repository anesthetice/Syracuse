// Modules
mod algorithms;
mod animation;
mod app;
mod data;
mod utils;

// Imports
use crate::app::App;
use color_eyre::eyre;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let app = App::load()?;
    app.run()
}
