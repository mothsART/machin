#[macro_use]
extern crate clap;
extern crate colored;
extern crate exitcode;
extern crate mime;
extern crate mime_guess;
extern crate resvg;
extern crate usvg;

use colored::*;
use std::io::{self, BufRead};
use std::path::Path;

use clap::{Arg, Command};

use machin::colored_err;
use machin::machconvert::*;

fn convert_files(prefix: Option<&str>, args: &ConvertArgs) {
    for line in io::stdin().lock().lines() {
        match line {
            Ok(l) => {
                let mut output_file = l.to_string();
                if let Some(_prefix) = prefix {
                    output_file = format!("{}{}", _prefix, output_file);
                }
                if !Path::new(&l).exists() {
                    colored_err!(format!("Input file \"{}\" doesn't exist", l));
                    continue;
                }
                let i_f = InputsFiles::new(&l, &output_file);
                match i_f.convert(args) {
                    Ok(_) => {}
                    Err(e) => {
                        colored_err!(e.to_string());
                    }
                };
            }
            Err(_) => {
                continue;
            }
        }
    }
}

fn main() {
    let matches = Command::new("machconvert")
        .version(crate_version!())
        .author(crate_authors!())
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
        .get_matches();

    let mut color = None;
    if let Some(color_value) = matches.value_of("color") {
        match color_value {
            "grayscale" => {
                color = Some(ConvertColor::Grayscale);
            }
            _e => {
                colored_err!(format!(
                    "color argument \"{}\" isn't a good value. There're only 1 option : grayscale",
                    _e,
                ));
                return;
            }
        }
    }

    let mut flip = None;
    if let Some(flip_value) = matches.value_of("flip") {
        match flip_value {
            "vertical" => {
                flip = Some(ConvertFlip::Vertical);
            }
            "horizontal" => {
                flip = Some(ConvertFlip::Horizontal);
            }
            _e => {
                colored_err!(format!(
                    "flip argument \"{}\" isn't a good value. There're only 2 options : vertical or horizontal", _e
                ));
                return;
            }
        }
    }

    let args = ConvertArgs {
        color,
        flip,
        rotate: matches.value_of("rotate"),
    };
    convert_files(matches.value_of("prefix"), &args);
}
