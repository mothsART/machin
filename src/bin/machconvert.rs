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

use machin::machconvert::*;
use machin::{colored_err, colored_success};

fn convert_files(prefix: Option<&str>, rotate_value: Option<&str>) {
    for line in io::stdin().lock().lines() {
        match line {
            Ok(l) => {
                let mut output_file = l.to_string();
                if let Some(_prefix) = prefix {
                    output_file = format!("{}{}", _prefix, output_file);
                }
                if !Path::new(&l).exists() {
                    colored_err!(format!(
                        "Input file \"{}\" doesn't exist", l
                    ));
                    continue;
                }
                let i_f = InputsFiles::new(&l, &output_file);
                match i_f.convert(rotate_value) {
                    Ok(r) => {
                        colored_success!(r);
                    }
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
        .about("Convert files but keep the same type")
        .arg_required_else_help(true)
        .arg(
            Arg::new("prefix")
                .short('p')
                .help("copy on new source with a file prefix")
                .takes_value(true),
        )
        .arg(
            Arg::new("rotate")
                .short('r')
                .help("rotate with degree. 90, 180 or 270.")
                .takes_value(true),
        )
        .get_matches();

    convert_files(matches.value_of("prefix"), matches.value_of("rotate"));
}
