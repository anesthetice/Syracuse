// Modules
mod add;
mod backup;
mod check_in;
mod check_out;
mod graph;
mod list;
mod prune;
mod reindex;
mod remove;
mod start;
mod sum;
mod today;
mod unindex;
mod update_add;
mod update_sub;

// Imports
use crate::{
    animation, config,
    data::{
        syrtime::{sec_to_pretty_string as stps, SyrDate, SyrSpan},
        Entries, Entry, IndexOptions,
    },
    dirs::Dirs,
    utils::{enter_clean_input_mode, exit_clean_input_mode, ARROW},
};
use clap::{command, value_parser, Arg, ArgAction, ArgGroup, ArgMatches, Command};
use color_eyre::eyre::{bail, Context, OptionExt};
use color_eyre::Result;
use crossterm::{event, style::Stylize};
use itertools::Itertools;
use jiff::civil::DateTime;
use jiff::Span;
use std::{
    io::{Read, Write},
    path::PathBuf,
    time::{Duration, Instant},
};

pub fn cli(entries: Entries, today: SyrDate, dt: DateTime) -> Result<()> {
    let command = command!().subcommands([
        add::subcommand(),
        list::subcommand(),
        remove::subcommand(),
        start::subcommand(),
        update_add::subcommand(),
        update_sub::subcommand(),
        today::subcommand(),
        backup::subcommand(),
        unindex::subcommand(),
        reindex::subcommand(),
        sum::subcommand(),
        prune::subcommand(),
        graph::subcommand(),
        check_in::subcommand(),
        check_out::subcommand(),
    ]);

    let arg_matches = command.get_matches();

    match arg_matches.subcommand() {
        Some(("add", arg_matches)) => add::process(arg_matches, &entries),
        Some(("list", arg_matches)) => list::process(arg_matches, &entries),
        Some(("remove", arg_matches)) => remove::process(arg_matches, &entries),
        Some(("start", arg_matches)) => start::process(arg_matches, &entries, &today),
        Some(("update-add", arg_matches)) => update_add::process(arg_matches, &entries, &today),
        Some(("update-sub", arg_matches)) => update_sub::process(arg_matches, &entries, &today),
        Some(("today", arg_matches)) => today::process(arg_matches, &entries, &today),
        Some(("backup", arg_matches)) => backup::process(arg_matches, &entries, &dt),
        Some(("unindex", arg_matches)) => unindex::process(arg_matches, &entries),
        Some(("reindex", arg_matches)) => reindex::process(arg_matches, &entries),
        Some(("sum", arg_matches)) => sum::process(arg_matches, &entries, &today),
        Some(("prune", arg_matches)) => prune::process(arg_matches, entries),
        Some(("graph", arg_matches)) => graph::process(arg_matches, entries, &today),
        Some(("check-in", arg_matches)) => check_in::process(arg_matches, &entries),
        Some(("check-out", arg_matches)) => check_out::process(arg_matches, &entries, &today),
        _ => Ok(()),
    }
}
