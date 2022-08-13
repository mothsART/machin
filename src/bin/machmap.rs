#[macro_use]
extern crate clap;
extern crate colored;
extern crate exitcode;
extern crate mime;
extern crate mime_guess;
extern crate resvg;
extern crate usvg;

use std::path::Path;
use std::process;

use clap::{Arg, Command};
use colored::*;

use machin::machmap::*;
use machin::{colored_err, colored_success};

include!("../machmap/cli.rs");

fn readlines() -> Vec<String> {
    use std::io::prelude::*;
    let stdin = std::io::stdin();
    let v = stdin.lock().lines().map(|x| x.unwrap()).collect();
    v
}

fn main() {
    let matches = build_machmap_cli("machmap", crate_version!(), crate_authors!()).get_matches();

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
    if let Some(extension) = matches.value_of("extension") {
        let fake_path = &format!("fake.{}", extension);
        let output_mime = mime_guess::from_path(fake_path);
        if output_mime.first().is_none() {
            colored_err!(format!(
                "Output file extension \"{}\" doesn't been reconize.",
                extension
            ));
            process::exit(exitcode::DATAERR);
        }

        for _l in readlines() {
            if !Path::new(&_l).exists() {
                colored_err!(format!("Input file \"{}\" doesn't exist", _l));
                continue;
            }
            let o_file = format!(
                "{}.{}",
                Path::new(&_l).file_stem().unwrap().to_str().unwrap(),
                &extension.to_string(),
            );
            let i_f = InputsFiles::new(&_l, &o_file);
            match i_f.mime_map() {
                Ok(r) => colored_success!(r),
                Err(e) => colored_err!(e.to_string()),
            };
        }
        return;
    }
    colored_err!("You must choose an extension file for conversion");
}
