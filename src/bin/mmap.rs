extern crate clap;
extern crate exitcode;
extern crate mime;
extern crate mime_guess;
extern crate resvg;
extern crate usvg;

use std::process;
use std::error::Error;
use std::path::Path;
use std::io::{self, BufRead};
use std::collections::HashMap;

use clap::{Arg, App};
use mime_guess::{Mime, MimeGuess};
use image::io::Reader as ImageReader;

use machin::{VERSION, AUTHOR, AUTHOR_MAIL};

use machin::errors::*;

fn svg_to_png_conversion<'a>(input_file: &'a str, output_file: &'a str) -> Result<(), Box<dyn Error + 'a>> {
    let opt = usvg::Options::default();
    
    let rtree = usvg::Tree::from_file(input_file, &opt)?;
    /* TODO : gérer proprement les erreurs tel que :
     * SVG has an invalid size => passer par un autre moteur de rendu ?
     * SVG data parsing failed cause the document does not have a root node :
     * Si le SVG n'a pas de xmlns="http://www.w3.org/2000/svg", le créer à la volée
    */
    let fit_to = usvg::FitTo::Zoom(1.0);
    let pixmap_size = fit_to.fit_to(rtree.svg_node().size.to_screen_size()).unwrap();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    resvg::render(&rtree, usvg::FitTo::Original, pixmap.as_mut()).unwrap();
    pixmap.save_png("output.png")?;
    Ok(())
}

struct SVGInputFile<'a> {
    map: HashMap<&'a str, Box<dyn Fn(&'a str, &'a str) -> Result<(), Box<dyn Error + 'a>>>>
}

impl<'a> SVGInputFile<'a> {
    fn new() -> SVGInputFile<'a> {
        let mut map: HashMap<&'a str, Box<dyn Fn(&'a str, &'a str) -> Result<(), Box<dyn Error + 'a>>>> = HashMap::new();
        map.insert("image/png", Box::new(svg_to_png_conversion));
        SVGInputFile {
            map: map
        }
    }

    fn mime_map(&self, input_file: &'a str, output_file: &'a str) -> Result<(), Box<dyn Error + 'a>> {
        let output_mime = mime_guess::from_path(output_file);
        println!("{:?}", output_mime.first());
        let e = UnSupportedError {
            input_file: input_file,
            output_ext: output_file
        };
        match &output_mime.first_raw() {
            Some(o_mime) => {
                match self.map.get(o_mime) {
                    Some(val) => {
                        val(input_file, output_file)
                    },
                    None => {
                        Err(Box::new(e))
                    }
                }
            },
            None => {
                Err(Box::new(e))
            }
        }
    }
}

fn svg_convert<'a>(input_file: &'a str, output_file: &'a str) -> Result<(), Box<dyn Error + 'a>> {
    println!("svg!");
    let svg_input_file = SVGInputFile::new();
    svg_input_file.mime_map(input_file, output_file)
}

fn markdown_convert<'a>(input_file: &'a str, output_file: &'a str)-> Result<(), Box<dyn Error + 'a>> {
    println!("markdown!");
    Ok(())
}

struct InputsFiles<'a> {
    map: HashMap<&'a str, Box<dyn Fn(&'a str, &'a str) -> Result<(), Box<dyn Error + 'a>>>>
}

impl<'a> InputsFiles<'a> {
    fn new() -> InputsFiles<'a> {
        let mut map: HashMap<&'a str, Box<dyn Fn(&'a str, &'a str) -> Result<(), Box<dyn Error + 'a>>>> = HashMap::new();
        map.insert("image/svg+xml", Box::new(svg_convert));
        map.insert("text/markdown", Box::new(markdown_convert));
        InputsFiles {
            map: map
        }
    }

    fn mime_map(&self, input_file: &'a str, output_file: &'a str) -> Result<(), Box<dyn Error + 'a>> {
        let input_mime = mime_guess::from_path(input_file);
        println!("{:?}", input_mime.first());
        let e = UnSupportedError {
            input_file: input_file,
            output_ext: output_file
        };
        match &input_mime.first_raw() {
            Some(i_mime) => {
                match self.map.get(i_mime) {
                    Some(val) => {
                        val(input_file, output_file)
                    },
                    None => {
                        Err(Box::new(e))
                    }
                }
            },
            None => {
                Err(Box::new(e))
            }
        }
    }
}

fn main() {
    let matches = App::new("mmap")
    .version(VERSION)
    .author(&*format!("{} <{}>", AUTHOR, AUTHOR_MAIL))
    .about("Transform files into another format")
    /*.arg(Arg::with_name("verbose")
        .short("v")
        .long("verbose")
        .multiple(true)
        .help("Sets the level of verbosity"))
    .arg(Arg::with_name("debug")
        .short("d")
        .long("debug")
        .help("print debug information verbosely"))
    */
    .arg(Arg::with_name("output")
        .short("o")
        .long("output")
        .help("output to a specific")
        .required(true)
        .takes_value(true))
    .get_matches();

    /*
    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }

    if matches.is_present("debug") {
        println!("Printing debug info...");
    } else {
        println!("Printing normally...");
    }*/
    
    if let Some(output_file) = matches.value_of("output") {
        let output_mime = mime_guess::from_path(output_file);
        match output_mime.first() {
            Some(i_m) => { },
            None => {
                eprintln!("Output file extension \"{}\" doesn't been reconize", output_file);
                process::exit(exitcode::DATAERR);
            }
        }
        for line in io::stdin().lock().lines() {
            match line {
                Ok(_l) => {
                    if Path::new(&_l).exists() == false {
                        println!("Input file \"{}\" doesn't exist", _l);
                        continue;
                    }
                    let i_f = InputsFiles::new();
                    if let Err(e) = i_f.mime_map(&_l, output_file) {
                         eprintln!("{}", e);
                    }
                },
                Err(_) => {
                    continue;
                }
            }
        }
    }
}
