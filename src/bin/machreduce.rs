#[macro_use]
extern crate clap;
extern crate colored;
extern crate exitcode;
extern crate mime;
extern crate mime_guess;
extern crate resvg;
extern crate usvg;
extern crate lopdf;

use colored::*;

use clap::{Arg, Command};

use machin::machreduce::*;

fn main() {
    let matches = Command::new("machreduce")
        .version(crate_version!())
        .author(crate_authors!())
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
        .get_matches();

    let mut direction = Direction::Vertical;

    if let Some(direction_value) = matches.value_of("direction") {
        match direction_value {
            "vertical" => { },
            "horizontal" => {
                direction = Direction::Horizontal;
            },
            _e => {
                eprintln!(
                    "{}",
                    format!("direction argument \"{}\" isn't a good value. There're only 2 options : vertical or horizontal", _e).white().on_red()
                );
                return;
            }
        }
    }

    match matches.value_of("output") {
        Some(output_file) => {
            let mut i_f = InputsFiles::new(output_file, direction);
            match i_f.reduce() {
                Ok(r) => println!("{}", r.white().on_green()),
                Err(e) => eprintln!("{}", e.to_string().white().on_red()),
            };
        }
        None => eprintln!("output file is required"),
    }
}
