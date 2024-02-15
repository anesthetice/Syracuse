use clap::{command, Arg, ArgAction, Command};


pub fn cli() -> clap::Command {
    let add_subcommand = Command::new("add")
        .alias("new")
        .about("used to add a new entry to syracuse")
        .long_about("used to add a new entry to syracuse, you can have aliases for entries, seperate these with commas")
        .arg(Arg::new("entry")
                .index(1)
                .num_args(1..10)
                .required(true)
                .help("add a new entry to syracuse")
                .long_help("")
                .action(ArgAction::Set)
            );
    
    let list_subcommand = Command::new("list")
        .alias("show")
        .about("displays all entries")
        .arg(Arg::new("full")
            .short('f')
            .long("full")
            .num_args(0)
            .required(false)
            .help("also displays the data contained by each entry")
            .action(ArgAction::SetTrue)
        );
    
    let remove_subcommand = Command::new("remove")
        .alias("delete")
        .about("deletes an entry")
        .arg(Arg::new("entry")
            .index(1)
            .required(true)
            .help("entry to remove")
            .action(ArgAction::Set)
        );

    let start_subcommand = Command::new("start")
        .about("starts the stopwatch for the given entry")
        .arg(Arg::new("entry")
            .index(1)
            .required(true)
            .help("entry to start")
            .action(ArgAction::Set)
        );
    
    command!()
        .subcommand(add_subcommand)
        .subcommand(list_subcommand)
        .subcommand(remove_subcommand)
        .subcommand(start_subcommand)
}