#[macro_use]
extern crate clap;
extern crate colored;
extern crate exitcode;
extern crate mime;
extern crate mime_guess;
extern crate resvg;
extern crate usvg;

use colored::*;
use std::path::Path;

use clap::{Arg, Command};

use machin::machconvert::{ConvertArgs, ConvertColor, ConvertFlip, InputsFiles};
use machin::{colored_err, readlines};

include!("../machconvert/cli.rs");

fn convert_files(prefix: Option<&str>, args: &ConvertArgs) {
    for line in readlines() {
        let mut output_file = line.to_string();
        if let Some(_prefix) = prefix {
            output_file = format!("{_prefix}{output_file}");
        }
        if !Path::new(&line).exists() {
            colored_err!(format!("Input file \"{line}\" doesn't exist"));
            continue;
        }
        let i_f = InputsFiles::new(&line, &output_file);
        if let Err(e) = i_f.convert(args) {
            colored_err!(e.to_string());
        };
    }
}

fn main() {
    let matches =
        build_machconvert_cli("machconvert", crate_version!(), crate_authors!()).get_matches();

    let mut color = None;
    if let Some("grayscale") = matches.get_one::<String>("color").map(|s| s.as_str()) {
        color = Some(ConvertColor::Grayscale);
    }

    let mut flip = None;
    let flip_value = matches.get_one::<String>("flip").map(|s| s.as_str());
    if let Some("vertical") = flip_value {
        flip = Some(ConvertFlip::Vertical);
    } else if let Some("horizontal") = flip_value {
        flip = Some(ConvertFlip::Horizontal);
    }

    let mut rotate: Option<u16> = None;

    if let Some(rotate_arg) = matches.get_one::<String>("rotate") {
        if let Ok(r) = rotate_arg.parse::<u16>() {
            rotate = Some(r);
        }
    }

    let args = ConvertArgs {
        color,
        flip,
        rotate,
    };
    convert_files(
        matches.get_one::<String>("prefix").map(|s| s.as_str()),
        &args,
    );
}
