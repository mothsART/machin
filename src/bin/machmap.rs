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

use clap::{Arg, Command};

use machin::mmap::*;

fn main() {
    let matches = Command::new("machmap")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Transform files into another format")
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
                eprintln!(
                    "The type of file \".{}\" is not yet supported.",
                    support_arg
                );
            }
        }
        return;
    }
    if let Some(output_file) = matches.value_of("output") {
        let output_mime = mime_guess::from_path(output_file);
        if output_mime.first().is_none() {
            eprintln!(
                "Output file extension \"{}\" doesn't been reconize",
                output_file
            );
            process::exit(exitcode::DATAERR);
        }
        for line in io::stdin().lock().lines() {
            match line {
                Ok(_l) => {
                    if !Path::new(&_l).exists() {
                        eprintln!(
                            "{}",
                            format!("Input file \"{}\" doesn't exist", _l)
                                .black()
                                .on_red()
                        );
                        continue;
                    }
                    let i_f = InputsFiles::new(&_l, output_file);
                    match i_f.mime_map() {
                        Ok(r) => {
                            println!("{}", r.black().on_green());
                        }
                        Err(e) => {
                            eprintln!("{}", e.to_string().black().on_red());
                        }
                    }
                }
                Err(_) => {
                    continue;
                }
            }
        }
    }
}
