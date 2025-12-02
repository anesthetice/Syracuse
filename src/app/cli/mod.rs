// Modules
mod add;
mod backup;
mod check_in;
mod check_out;
mod list;
/*
mod misc;
mod prune;
mod reindex;
mod remove;
mod start;
mod sum;
mod today;
mod unindex;
mod update_add;
mod update_sub;
mod week;
*/

// Re-exports
use check_in::CIN_FILENAME;

// Imports
use super::App;
use crate::{
    animation,
    data::{
        Entries, Entry, EntryCore, IEntry, IndexOptions, SyrDate, SyrSpan, TimeFormatting, UEntry,
        WeekdayFormatting,
    },
    utils::{ARROW, ARROWHEAD, enter_clean_input_mode, exit_clean_input_mode},
};
use clap::{Arg, ArgAction, ArgGroup, ArgMatches, Command, value_parser};
use clap_complete::{Shell, generate};
use color_eyre::Result;
use color_eyre::eyre::{Context, OptionExt, bail};
use crossterm::{event, style::Stylize};
use itertools::Itertools;
use jiff::ToSpan;
use jiff::civil::{DateTime, Weekday};
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::PathBuf,
    time::{Duration, Instant},
};

pub fn build_cli() -> Command {
    Command::new("syr").subcommands([
        check_in::subcommand(),
        check_out::subcommand(),
        update_add::subcommand(),
        update_sub::subcommand(),
        add::subcommand(),
        //list::subcommand(),
        //remove::subcommand(),
        //start::subcommand(),

        //today::subcommand(),
        backup::subcommand(),
        //unindex::subcommand(),
        //reindex::subcommand(),
        //sum::subcommand(),
        //prune::subcommand(),
        //graph::subcommand(),
        //week::subcommand(),
        //gen_completions::subcommand(),
    ])
}
