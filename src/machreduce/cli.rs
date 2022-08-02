use clap::{Arg, Command};

pub fn build_cli(
    name: &'static str,
    version: &'static str,
    authors: &'static str,
) -> Command<'static> {
    Command::new(name)
        .version(version)
        .author(authors)
        .about("Reduce a list of files to one")
        .arg_required_else_help(true)
        .arg(
            Arg::new("output")
                .short('o')
                .help("output to a specific file (like result.zip)")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::new("direction")
                .short('d')
                .help("direction : horizontal or vertical (vertical by default)")
                .takes_value(true),
        )
}
