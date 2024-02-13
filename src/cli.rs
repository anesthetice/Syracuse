use clap::{command, Arg, ArgAction, Command};


pub fn cli() -> clap::Command {
    command!()
        .subcommand(Command::new("add")
            .arg(Arg::new("entry")
                .index(1)
                .num_args(1..10)
                .required(true)
                .help("adds a new entry to syracuse")
                .long_help("adds a new entry to syracuse, you can add an alias to the entry by separating each one with ','")
                .action(ArgAction::Set)
            )
        )
        .subcommand(Command::new("list")
            .arg(Arg::new("full")
            .short('f')
            .long("full")
            .num_args(0)
            .required(false)
            .help("lists every entry with the data they contain")
            .action(ArgAction::SetTrue)
            )
        )
        .subcommand(Command::new("remove")
            .arg(Arg::new("entry")
                .index(1)
                .required(true)
                .help("removes an entry from syracuse")
                .action(ArgAction::Set)
            )
        )
        .subcommand(Command::new("start")
            .arg(Arg::new("entry")
                .index(1)
                .required(true)
                .help("starts the stopwatch for a given entry")
                .action(ArgAction::Set)
            )
        )
}