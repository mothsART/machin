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

use machin::machreduce::*;
use machin::readlines;
use machin::{colored_err, colored_success};

include!("../machreduce/cli.rs");

fn main() {
    let matches =
        build_machreduce_cli("machreduce", crate_version!(), crate_authors!()).get_matches();

    let mut direction = Direction::Vertical;

    if let Some(direction_value) = matches.get_one::<String>("direction").map(|s| s.as_str()) {
        match direction_value {
            "vertical" => {}
            "horizontal" => {
                direction = Direction::Horizontal;
            }
            _e => { }
        }
    }

    match matches.get_one::<String>("output").map(|s| s.as_str()) {
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
