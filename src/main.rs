mod cli;
mod data;

fn main() {
    let command = cli::cli();
    let matches = command.get_matches();
    matches.subcommand_matches("add").unwrap().get_many::<String>("entry").unwrap().for_each(|e| {println!("{e}")});
}