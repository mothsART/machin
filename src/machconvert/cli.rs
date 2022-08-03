pub fn build_machconvert_cli(
    name: &'static str,
    version: &'static str,
    authors: &'static str,
) -> Command<'static> {
    Command::new(name)
        .version(version)
        .author(authors)
        .about("Convert files but keep the same type (priority arguments are important)")
        .arg_required_else_help(true)
        .arg(
            Arg::new("prefix")
                .short('p')
                .help("copy on new source with a file prefix")
                .takes_value(true),
        )
        .arg(
            Arg::new("color")
                .short('c')
                .help("color (priority 1) : grayscale")
                .takes_value(true),
        )
        .arg(
            Arg::new("flip")
                .short('f')
                .help("flip (priority 2) : horizontal or vertical")
                .takes_value(true),
        )
        .arg(
            Arg::new("rotate")
                .short('r')
                .help("rotate (priority 3) with degree. 90, 180 or 270.")
                .takes_value(true),
        )
}
