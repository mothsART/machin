#[macro_use]
extern crate clap;
extern crate colored;
extern crate exitcode;
extern crate lopdf;
extern crate mime;
extern crate mime_guess;
extern crate resvg;
extern crate usvg;

use clap::{Arg, Command};
use colored::*;

use machin::readlines;
use machin::machreduce::*;
use machin::{colored_err, colored_success};

include!("../machreduce/cli.rs");

fn main() {
    let matches =
        build_machreduce_cli("machreduce", crate_version!(), crate_authors!()).get_matches();

    let mut direction = Direction::Vertical;

    if let Some(direction_value) = matches.value_of("direction") {
        match direction_value {
            "vertical" => {}
            "horizontal" => {
                direction = Direction::Horizontal;
            }
            _e => {
                colored_err!(format!(
                    "direction argument \"{}\" isn't a good value. There're only 2 options : vertical or horizontal", _e
                ));
                return;
            }
        }
    }

    match matches.value_of("output") {
        Some(output_file) => {
            let r = readlines();
            let mut i_f = InputsFiles::new(&r, output_file, direction);
            match i_f.reduce() {
                Ok(r) => colored_success!(r),
                Err(e) => colored_err!(e.to_string()),
            };
        }
        None => colored_err!("output file is required"),
    }
}
