use clap::{command, Arg, ArgAction, Command};

pub fn cli() -> clap::Command {
    let add_subcommand = Command::new("add")
        .alias("new")
        .about("used to add a new entry to syracuse")
        .long_about("used to add a new entry to syracuse, you can have aliases for entries, seperate these with spaces")
        .arg(Arg::new("entry")
                .index(1)
                .num_args(1..10)
                .required(true)
                .help("entry to add")
                .long_help("")
                .action(ArgAction::Set)
            );

    let list_subcommand = Command::new("list")
        .aliases(["view", "display", "show"])
        .about("display all entries")
        .arg(
            Arg::new("full")
                .short('f')
                .short_alias('a')
                .long("full")
                .alias("all")
                .num_args(0)
                .required(false)
                .help("also displays the data contained by each entry")
                .action(ArgAction::SetTrue),
        );

    let remove_subcommand = Command::new("remove")
        .aliases(["delete", "del"])
        .about("remove an entry")
        .arg(
            Arg::new("entry")
                .index(1)
                .required(true)
                .help("entry to remove")
                .action(ArgAction::Set),
        );

    let start_subcommand = Command::new("start")
        .aliases(["s", "r", "run", "go", "launch"])
        .about("starts the stopwatch for the given entry")
        .arg(
            Arg::new("entry")
                .index(1)
                .required(true)
                .help("entry to start")
                .action(ArgAction::Set),
        );

    let update_subcommand = Command::new("update")
        .about("updates an entry's data")
        .arg(
            Arg::new("entry")
                .index(1)
                .required(true)
                .help("entry to update")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("date")
                .required(false)
                .help("updates specified date")
                .long_help("updates specified date, defaults to today")
                .short('d')
                .long("date")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("hour")
                .required(false)
                .help("hours to add or subtract")
                .short('t')
                .long("hour")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("minute")
                .required(false)
                .help("minutes to add or subtract")
                .short('m')
                .long("minute")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("second")
                .required(false)
                .help("secondss to add or subtract")
                .short('s')
                .long("second")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("negative")
                .required(false)
                .help("subtract the provided time instead of adding it")
                .short('n')
                .long("negative")
                .action(ArgAction::SetTrue),
        );

    let graph_subcommand = Command::new("graph")
        .alias("export")
        .about("create a graph")
        .arg(
            Arg::new("all")
                .short('a')
                .short('f')
                .long("all")
                .alias("full")
                .exclusive(true)
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("single")
                .exclusive(true)
                .short('s')
                .long("single")
                .required(false)
                .action(ArgAction::Set),
        );

    command!()
        .subcommand(add_subcommand)
        .subcommand(list_subcommand)
        .subcommand(remove_subcommand)
        .subcommand(start_subcommand)
        .subcommand(update_subcommand)
        .subcommand(graph_subcommand)
}
