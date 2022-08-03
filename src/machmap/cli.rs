pub fn build_machmap_cli(
    name: &'static str,
    version: &'static str,
    authors: &'static str,
) -> Command<'static> {
    Command::new(name)
        .version(version)
        .author(authors)
        .about("Transform files into another format")
        .arg_required_else_help(true)
        .arg(
            Arg::new("extension")
                .short('e')
                .help("output to a specific extension name (like png)")
                .takes_value(true),
        )
        .arg(
            Arg::new("support")
                .short('s')
                .help("return list of supporting conversion")
                .takes_value(true),
        )
}
