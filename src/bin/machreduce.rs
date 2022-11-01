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

    if let Some(output_file) = matches.get_one::<PathBuf>("output") {
        let r = readlines();
        let o = output_file.display().to_string();

        let mut i_f = InputsFiles::new(&r, &o, direction);
        match i_f.reduce() {
            Ok(r) => colored_success!(r),
            Err(e) => colored_err!(e.to_string()),
        };
    }
}
