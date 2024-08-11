use anyhow::{anyhow, Context};
use clap::{command, value_parser, Arg, ArgAction, ArgGroup, ArgMatches, Command};
use crossterm::{event, style::Stylize};
use std::{
    path::PathBuf,
    time::{Duration, Instant},
};
use crate::{
    animation, config,
    data::{
        graphing,
        internal::{Entries, Entry, IndexOptions},
        syrtime::syrdate::SyrDate,
    },
    utils::{enter_clean_input_mode, exit_clean_input_mode},
};
use crate::data::syrtime::blocs::sec_to_pretty_string;
use jiff::civil::Time;
use jiff::civil::DateTime;
use jiff::Span;
use tracing::{debug, info, warn, error};
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

pub fn cli(entries: Entries, today: SyrDate, dt: DateTime) -> anyhow::Result<()> {
    let command = command!()
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
            add::subcommand(),
            list::subcommand(),
            remove::subcommand(),
            start::subcommand(),
            update::subcommand(),
            today::subcommand(),
            backup::subcommand(),
            unindex::subcommand(),
            reindex::subcommand(),
            sum::subcommand(),
            prune_subcommand(),
            graph_subcommand(),
        ]);

    let arg_matches = command.get_matches();

    match arg_matches.subcommand() {
        Some(("add", arg_matches)) => add::process(arg_matches, &entries),
        Some(("list", arg_matches)) => list::process(arg_matches, &entries),
        Some(("remove", arg_matches)) => remove::process(arg_matches, &entries),
        Some(("start", arg_matches)) => start::process(arg_matches, &entries, &today),
        Some(("update", arg_matches)) => update::process(arg_matches, &entries, &today),
        Some(("today", arg_matches)) => today::process(arg_matches, &entries, &today),
        Some(("backup", arg_matches)) => backup::process(arg_matches, &entries, &dt),
        Some(("unindex", arg_matches)) => unindex::process(arg_matches, &entries),
        Some(("reindex", arg_matches)) => reindex::process(arg_matches, &entries),
        Some(("sum", arg_matches)) => sum::process(arg_matches, &entries, &today),
        Some(("", arg_matches)) => ,
        Some(("", arg_matches)) => ,
        Some(("", arg_matches)) => ,
        Some(_) => Ok(()),
        None => Ok(()),
    }
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
