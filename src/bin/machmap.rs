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
use std::process;
use std::ffi::OsStr;

use clap::{Arg, Command};

use machin::machmap::*;
use machin::{colored_err, colored_success};

fn readlines() -> Vec<String> {
    use std::io::prelude::*;
    let stdin = std::io::stdin();
    let v = stdin.lock().lines().map(|x| x.unwrap()).collect();
    v
}

fn main() {
    let matches = Command::new("machmap")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Transform files into another format")
        .arg_required_else_help(true)
        .arg(
            Arg::new("output")
                .short('o')
                .help("output to a specific file pattern (like *.png)")
                .takes_value(true),
        )
        .arg(
            Arg::new("support")
                .short('s')
                .help("return list of supporting conversion")
                .takes_value(true),
        )
        .get_matches();

    if let Some(support_arg) = matches.value_of("support") {
        let fake_path = &format!("fake.{}", support_arg);
        let i = InputsFiles::new(fake_path, "fake");
        match i.support() {
            Ok(r) => {
                println!("The type of file \".{}\" support :", support_arg);
                println!("{}", r);
            }
            Err(_e) => {
                colored_err!(format!(
                    "The type of file \".{}\" is not yet supported.",
                    support_arg
                ));
            }
        }
        return;
    }
    if let Some(output_file) = matches.value_of("output") {
        let output_mime = mime_guess::from_path(output_file);
        if output_mime.first().is_none() {
            colored_err!(format!(
                "Output file extension \"{}\" doesn't been reconize.",
                output_file
            ));
            process::exit(exitcode::DATAERR);
        }

        let mut is_static_output_file = true;
        let mut output_extension = "";
        if let Some(prefix) = Path::new(&output_file).file_stem() {
            if prefix == "*" {
                is_static_output_file = false;
                output_extension = Path::new(&output_file).extension().and_then(OsStr::to_str).unwrap();
            }
        }

        let lines = readlines();
        if is_static_output_file == true && lines.len() >= 2 {
            colored_err!(format!(
                "Output file extension \"{}\" is unique. You can't choise it for every input files.",
                output_file
            ));
            process::exit(exitcode::DATAERR);
        }

        for _l in lines {
            let mut o_file = output_file;
            let tmp_file;

            if !Path::new(&_l).exists() {
                colored_err!(format!(
                    "Input file \"{}\" doesn't exist", _l
                ));
                continue;
            }
            if !is_static_output_file {
                tmp_file = format!(
                    "{:?}.{:?}",
                    Path::new(&_l).file_stem().unwrap(),
                    &output_extension,
                );
                o_file = &tmp_file;
            }
            let i_f = InputsFiles::new(&_l, &o_file);
            match i_f.mime_map() {
                Ok(r) => {
                    colored_success!(r);
                }
                Err(e) => {
                    colored_err!(e.to_string());
                }
            };
        }
    }
}
