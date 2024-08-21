use crate::data::syrtime::blocs::sec_to_pretty_string;
use crate::{
    animation, config,
    data::{
        internal::{Entries, Entry, IndexOptions},
        syrtime::syrdate::SyrDate,
        syrtime::syrspan::SyrSpan,
    },
    utils::{enter_clean_input_mode, exit_clean_input_mode},
};
use anyhow::{anyhow, Context};
use clap::{command, value_parser, Arg, ArgAction, ArgGroup, ArgMatches, Command};
use crossterm::{event, style::Stylize};
use jiff::civil::DateTime;
use jiff::Span;
use std::{
    path::PathBuf,
    time::{Duration, Instant},
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
            prune::subcommand(),
            graph::subcommand(),
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
        Some(("prune", arg_matches)) => prune::process(arg_matches, entries),
        Some(("graph", arg_matches)) => graph::process(arg_matches, entries, &today),
        _ => Ok(()),
    }
}
