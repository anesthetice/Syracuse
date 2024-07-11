use add::add_subcommand;
use anyhow::Context;
use backup::backup_subcommand;
use clap::{command, value_parser, Arg, ArgAction, ArgGroup, ArgMatches, Command};
use crossterm::{event, style::Stylize};
use graph::graph_subcommand;
use list::list_subcommand;
use prune::prune_subcommand;
use reindex::reindex_subcommand;
use remove::remove_subcommand;
use start::start_subcommand;
use std::{
    path::PathBuf,
    time::{Duration, Instant},
};
use sum::sum_subcommand;
use time::{OffsetDateTime, Time};
use today::today_subcommand;
use unindex::unindex_subcommand;
use update::update_subcommand;

use crate::{
    animation, config,
    data::{
        graphing,
        internal::{Entries, Entry, IndexOptions},
        syrtime::{ns_to_pretty_string, SyrDate},
    },
    error, info,
    utils::{enter_clean_input_mode, exit_clean_input_mode},
    warn,
};

mod add;
mod backup;
mod graph;
mod list;
mod prune;
mod reindex;
mod remove;
mod start;
mod sum;
mod today;
mod unindex;
mod update;

pub fn cli() -> clap::Command {
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
            add_subcommand(),
            list_subcommand(),
            remove_subcommand(),
            start_subcommand(),
            update_subcommand(),
            today_subcommand(),
            backup_subcommand(),
            unindex_subcommand(),
            reindex_subcommand(),
            sum_subcommand(),
            prune_subcommand(),
            graph_subcommand(),
        ])
}

// might not be the prettiest way of doing things
// but it's not so bad, and it lets me keep main.rs pretty clean
pub enum ProcessOutput {
    Continue(Option<Entries>),
    Terminate,
}

use ProcessOutput as PO;

pub use add::process_add_subcommand;
pub use backup::process_backup_subcommand;
pub use graph::process_graph_subcommand;
pub use list::process_list_subcommand;
pub use prune::process_prune_subcommand;
pub use reindex::process_reindex_subcommand;
pub use remove::process_remove_subcommand;
pub use start::process_start_subcommand;
pub use sum::process_sum_subcommand;
pub use today::process_today_subcommand;
pub use unindex::process_unindex_subcommand;
pub use update::process_update_subcommand;
