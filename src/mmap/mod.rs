use std::collections::HashMap;

use std::error::Error;
use crate::errors::*;

pub mod svg;
//use crate::mmap::svg::SVGTo;

pub trait IFile<'a> {
    fn mime_map(&self) -> Result<(), Box<dyn Error + 'a>>;
}

#[macro_export]
macro_rules! create_input {
    ($struct_name:ident, $convert_trait:ident) => {
        struct $struct_name<'a> {
            input_file: &'a str,
            output_file: &'a str,
            map: HashMap<&'a str, Box<dyn $convert_trait<'a> +'a>>
        }
        
        impl<'a> IFile<'a> for $struct_name<'a> {
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
    }
}

pub struct InputsFiles<'a> {
    pub input_file: &'a str,
    pub output_file: &'a str,
    pub map: HashMap<&'a str, Box<dyn IFile<'a> + 'a>>
}

impl<'a> InputsFiles<'a> {
    pub fn new(input_file: &'a str, output_file: &'a str) -> InputsFiles<'a> {
        let mut map: HashMap<&'a str, Box<dyn IFile +'a>> = HashMap::new();
        let svg = SVGInputFile::new(input_file, output_file);
        //let markdown = MarkdownInputFile::new(input_file, output_file);
        map.insert("image/svg+xml", Box::new(svg));
        //map.insert("text/markdown", Box::new(markdown));
        InputsFiles {
            input_file: input_file,
            output_file: output_file,
            map: map
        }
    }

    pub fn mime_map(&self) -> Result<(), Box<dyn Error + 'a>> {
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

trait InputTo<'a> {
    fn convert(&self) -> Result<(), Box<dyn Error + 'a>>;
}

create_input!(SVGInputFile, InputTo);
create_input!(MarkdownInputFile, InputTo);
