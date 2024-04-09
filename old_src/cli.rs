use clap::{command, Arg, ArgAction, Command};

pub fn cli() -> clap::Command {
    let add_subcommand = Command::new("add")
        .alias("new")
        .about("Add a new entry to syracuse\naliases: 'new'")
        .long_about("This subcommand is used to add a new entry to syracuse, entries are case-insensitive and can have aliases\naliases: 'new'")
        .arg(Arg::new("entry")
                .index(1)
                .num_args(1..10)
                .required(true)
                .help("entry to add")
                .long_help("entry to add\ne.g. 'add math-201 analysis' will add an entry titled 'MATH-201' with the alias 'ANALYSIS'")
                .action(ArgAction::Set)
            );

    let list_subcommand = Command::new("list")
        .about("Lists out all entries")
        .long_about("This subcommand is used to list out all entries stored in syracuse.json")
        .arg(
            Arg::new("full")
                .short('f')
                .short_alias('a')
                .long("full")
                .alias("all")
                .num_args(0)
                .required(false)
                .help("prints out the data associated with each entry as well")
                .action(ArgAction::SetTrue),
        );

    let remove_subcommand = Command::new("remove")
        .aliases(["delete", "del"])
        .about("Removes a single entry\naliases: 'delete', 'del'")
        .long_about("This subcommand is used to remove a single entry at a time from syracuse.json\naliases: 'delete', 'del'")
        .arg(
            Arg::new("entry")
                .index(1)
                .required(true)
                .help("entry to remove")
                .action(ArgAction::Set),
        );

    let start_subcommand = Command::new("start")
        .aliases(["s", "r", "run", "go", "launch", "begin"])
        .about("Starts the daily stopwatch for the given entry")
        .long_about("This subcommand is used to start counting up the time spent today on the given entry, will progressively update syracuse.json\naliases: 's', 'r', 'run', 'go', 'launch', 'begin'")
        .arg(
            Arg::new("entry")
                .index(1)
                .required(true)
                .help("entry to start")
                .action(ArgAction::Set),
        );

    let update_subcommand = Command::new("update")
        .about("Manually updates the time of an entry")
        .long_about("This subcommand is used to manually increase or decrease the time associated with an entry on a given day")
        .arg(
            Arg::new("entry")
                .index(1)
                .required(true)
                .help("entry to update")
                .action(ArgAction::Set),
        )
        .subcommand(
            Command::new("add").alias("a").index(2)
            Command::new("sub").alias("s").index(2)
        )
        .arg(
            Arg::new("date")
                .required(false)
                .help("the targeted date")
                .long_help("the targeted date, defaults to today")
                .short('d')
                .long("date")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("hour")
                .required(false)
                .help("the number of hours to add or subtract")
                .short('h')
                .short_alias('t')
                .long("hour")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("minute")
                .required(false)
                .help("the number of minutes to add or subtract")
                .short('m')
                .long("minute")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("second")
                .required(false)
                .help("the number of seconds to add or subtract")
                .short('s')
                .long("second")
                .action(ArgAction::Set),
        );

    let graph_subcommand = Command::new("graph")
        .about("Creates a graph")
        .long_about("This subcommand is used to graph the entries within a specified time frame")
        .arg(
            Arg::new("all")
                .help("graphs all entries")
                .exclusive(true)
                .short('a')
                .short('f')
                .long("all")
                .alias("full")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("single")
                .help("graphs a single specified entry")
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
