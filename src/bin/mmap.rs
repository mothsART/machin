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
use image::io::Reader as ImageReader;

use machin::{VERSION, AUTHOR, AUTHOR_MAIL};

use machin::errors::*;

trait SVGTo<'a> {
    fn convert(&self) -> Result<(), Box<dyn Error + 'a>>;
}

struct SVGToPNG<'a> {
    input_file: &'a str,
    output_file: &'a str
}

impl<'a> SVGToPNG<'a> {
    fn new(input_file: &'a str, output_file: &'a str) -> SVGToPNG<'a> {
        SVGToPNG {
            input_file: input_file,
            output_file: output_file
        }
    }
}

impl<'a> SVGTo<'a> for SVGToPNG<'a> {
    fn convert(&self) -> Result<(), Box<dyn Error + 'a>> {
        println!("conversion svg to png");
        let opt = usvg::Options::default();
        
        let rtree = usvg::Tree::from_file(self.input_file, &opt)?;
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
}

struct SVGToJPG<'a> {
    input_file: &'a str,
    output_file: &'a str
}

impl<'a> SVGToJPG<'a> {
    fn new(input_file: &'a str, output_file: &'a str) -> SVGToJPG<'a> {
        SVGToJPG {
            input_file: input_file,
            output_file: output_file
        }
    }
}

impl<'a> SVGTo<'a> for SVGToJPG<'a> {
    fn convert(&self) -> Result<(), Box<dyn Error + 'a>> {
        println!("conversion svg to jpg");
        Ok(())
    }
}

// --------------------

trait IFile<'a> {
    fn mime_map(&self) -> Result<(), Box<dyn Error + 'a>>;
}

struct SVGInputFile<'a> {
    input_file: &'a str,
    output_file: &'a str,
    map: HashMap<&'a str, Box<dyn SVGTo<'a> +'a>>
}

impl<'a> SVGInputFile<'a> {
    fn new(input_file: &'a str, output_file: &'a str) -> SVGInputFile<'a> {
        let png = SVGToPNG::new(input_file, output_file);
        let jpg = SVGToJPG::new(input_file, output_file);

        let mut map: HashMap<&'a str, Box<dyn SVGTo<'a> +'a>> = HashMap::new();
        map.insert("image/png", Box::new(png));
        map.insert("image/jpeg", Box::new(jpg));
        SVGInputFile {
            input_file: input_file,
            output_file: output_file,
            map: map
        }
    }
}


impl<'a> IFile<'a> for SVGInputFile<'a> {
    fn mime_map(&self) -> Result<(), Box<dyn Error + 'a>> {
        let output_mime = mime_guess::from_path(self.output_file);
        println!("output => {:?}", output_mime.first());
        let e = UnSupportedError {
            input_file: self.input_file,
            output_ext: self.output_file
        };
        match &output_mime.first_raw() {
            Some(i_mime) => {
                match self.map.get(i_mime) {
                    Some(val) => {
                        val.convert()
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


/*
struct MarkdownInputFile {
    
}

impl IFile for MarkdownInputFile {
    fn get(&self) {
        println!("hehe");
    }
}
*/

struct InputsFiles<'a> {
    input_file: &'a str,
    output_file: &'a str,
    map: HashMap<&'a str, Box<dyn IFile<'a> + 'a>>
}

impl<'a> InputsFiles<'a> {
    fn new(input_file: &'a str, output_file: &'a str) -> InputsFiles<'a> {
        let mut map: HashMap<&'a str, Box<dyn IFile +'a>> = HashMap::new();
        let svg = SVGInputFile::new(input_file, output_file);
        //let markdown = MarkdownInputFile {};
        map.insert("image/svg+xml", Box::new(svg));
        //map.insert("text/markdown", Box::new(markdown));
        InputsFiles {
            input_file: input_file,
            output_file: output_file,
            map: map
        }
    }

    fn mime_map(&self) -> Result<(), Box<dyn Error + 'a>> {
        let input_mime = mime_guess::from_path(self.input_file);
        println!("input => {:?}", input_mime.first());
        let e = UnSupportedError {
            input_file: self.input_file,
            output_ext: self.output_file
        };
        match &input_mime.first_raw() {
            Some(i_mime) => {
                match self.map.get(i_mime) {
                    Some(val) => {
                        val.mime_map()
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
    .arg(Arg::with_name("output")
        .short("o")
        .long("output")
        .help("output to a specific")
        .required(true)
        .takes_value(true))
    .get_matches();

    if let Some(output_file) = matches.value_of("output") {
        let output_mime = mime_guess::from_path(output_file);
        if let None = output_mime.first() {
            eprintln!("Output file extension \"{}\" doesn't been reconize", output_file);
            process::exit(exitcode::DATAERR);
        }
        for line in io::stdin().lock().lines() {
            match line {
                Ok(_l) => {
                    if Path::new(&_l).exists() == false {
                        println!("Input file \"{}\" doesn't exist", _l);
                        continue;
                    }
                    let i_f = InputsFiles::new(&_l, output_file);
                    if let Err(e) = i_f.mime_map() {
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
