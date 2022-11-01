pub fn build_machconvert_cli(
    name: &'static str,
    version: &'static str,
    authors: &'static str,
) -> Command {
    Command::new(name)
        .version(version)
        .author(authors)
        .about("Convert files but keep the same type (priority arguments are important)")
        .arg_required_else_help(true)
        .args([
            Arg::new("prefix")
                .short('p')
                .value_parser(clap::value_parser!(String))
                .help("copy on new source with a file prefix"),
            Arg::new("color")
                .short('c')
                .value_parser(["grayscale"])
                .help("color (priority 1)"),
            Arg::new("flip")
                .short('f')
                .value_parser(["horizontal", "vertical"])
                .help("flip (priority 2)"),
            Arg::new("rotate")
                .short('r')
                .value_parser(["90", "180", "270"])
                .help("rotate (priority 3) with degree. 90, 180 or 270.")
        ])
}
