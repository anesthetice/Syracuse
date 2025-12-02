use super::*;

pub(super) fn subcommand() -> Command {
    Command::new("gen-completions")
        .about("Generate completions for your desired shell")
        .long_about("This subcommand is used to generate shell completions for the selected shell, outputs to stdout")
        .arg(
            Arg::new("shell")
                .index(1)
                .required(true)
                .help("The shell to target")
                .action(ArgAction::Set)
                .value_parser(value_parser!(Shell)),
        )
}

pub fn process(arg_matches: &ArgMatches) -> Result<()> {
    if let Some(generator) = arg_matches.get_one::<Shell>("shell").copied() {
        let mut cmd = build_cli();
        let bin_name = cmd.get_name().to_string();
        generate(generator, &mut cmd, bin_name, &mut std::io::stdout());
        Ok(())
    } else {
        bail!("No valid shell (generator) provided")
    }
}
