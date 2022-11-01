pub fn build_machmap_cli(
    name: &'static str,
    version: &'static str,
    authors: &'static str,
) -> Command {
    Command::new(name)
        .version(version)
        .author(authors)
        .about("Transform files into another format")
        .arg_required_else_help(true)
        .args([
            Arg::new("extension")
                .short('e')
                .help("output to a specific extension name (like png)"),
            Arg::new("support")
                .short('s')
                .help("return list of supporting conversion"),
        ])
}
