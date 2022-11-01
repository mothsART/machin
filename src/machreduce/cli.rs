use std::path::PathBuf;

pub fn build_machreduce_cli(
    name: &'static str,
    version: &'static str,
    authors: &'static str,
) -> Command {

    Command::new(name)
        .version(version)
        .author(authors)
        .about("Reduce a list of files to one")
        .arg_required_else_help(true)
        .args([
            Arg::new("output")
                .short('o')
                .value_parser(clap::value_parser!(PathBuf))
                .help("output to a specific file (like result.zip)")
                .required(true),
            Arg::new("direction")
                .short('d')
                .value_parser(["horizontal", "vertical"])
                .help("direction : horizontal or vertical (vertical by default)"),
        ])
}
