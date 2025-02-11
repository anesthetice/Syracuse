// Imports
use crate::data::syrtime::syrdate::SyrDate;
use crossterm::{
    cursor, execute,
    style::{StyledContent, Stylize},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::stdout;

static ARROW: &str = "――>";

pub fn enter_clean_input_mode() {
    let _ =
        enable_raw_mode().map_err(|err| eprintln!("Warning, Failed to enable raw mode: '{err}'"));
    let _ = execute!(stdout(), cursor::Hide)
        .map_err(|err| eprintln!("Warning, Failed to hide cursor: '{err}'"));
}

pub fn exit_clean_input_mode() {
    let _ = execute!(stdout(), cursor::Show)
        .map_err(|err| eprintln!("Warning: Failed to show cursor: '{err}'"));
    let _ =
        disable_raw_mode().map_err(|err| eprintln!("Warning: Failed to disable raw mode: '{err}'"));
}

pub fn print_datearrow<T1, T2>(date: &SyrDate, pre: T1, post: T2, color: &str)
where
    T1: std::fmt::Display,
    T2: std::fmt::Display,
{
    let arrow = get_arrow(color);
    println!("{date}  :  {pre}  {arrow}  {post}")
}

pub fn print_arrow<T1>(post: T1, color: &str)
where
    T1: std::fmt::Display,
{
    let arrow = get_arrow(color);
    println!("{arrow}  {post}")
}

fn get_arrow(color: &str) -> StyledContent<&str> {
    match color {
        "red" => ARROW.red(),
        "green" => ARROW.green(),
        "blue" => ARROW.blue(),
        "cyan" => ARROW.cyan(),
        _ => ARROW.stylize(),
    }
}
