use std::collections::HashMap;
use std::error::Error;

use crate::errors::*;

pub mod jpg;
pub mod markdown;
pub mod png;
pub mod svg;

pub trait IFile<'a> {
    fn support(&self) -> Result<String, Box<dyn Error + 'a>>;
    fn mime_map(&self) -> Result<String, Box<dyn Error + 'a>>;
}

#[macro_export]
macro_rules! create_input {
    ($struct_name:ident, $convert_trait:ident) => {
        struct $struct_name<'a> {
            input_file: &'a str,
            output_file: &'a str,
            map: HashMap<&'a str, Box<dyn $convert_trait<'a> + 'a>>,
        }

        impl<'a> IFile<'a> for $struct_name<'a> {
            fn support(&self) -> Result<String, Box<dyn Error + 'a>> {
                let mut result = "".to_string();
                for (key, _) in &self.map {
                    result = format!("{}{}\n", result, key);
                }
                Ok(result)
            }

            fn mime_map(&self) -> Result<String, Box<dyn Error + 'a>> {
                let output_mime = mime_guess::from_path(self.output_file);
                let e = UnSupportedError {
                    input_file: self.input_file,
                    output_ext: self.output_file,
                };
                match &output_mime.first_raw() {
                    Some(i_mime) => match self.map.get(i_mime) {
                        Some(val) => val.convert(),
                        None => Err(Box::new(e)),
                    },
                    None => Err(Box::new(e)),
                }
            }
        }
    };
}

pub struct InputsFiles<'a> {
    pub input_file: &'a str,
    pub output_file: &'a str,
    pub map: HashMap<&'a str, Box<dyn IFile<'a> + 'a>>,
}

impl<'a> InputsFiles<'a> {
    pub fn new(input_file: &'a str, output_file: &'a str) -> InputsFiles<'a> {
        let mut map: HashMap<&'a str, Box<dyn IFile + 'a>> = HashMap::new();
        let svg = SVGInputFile::new(input_file, output_file);
        let jpg = JPGInputFile::new(input_file, output_file);
        let png = PNGInputFile::new(input_file, output_file);
        let markdown = MarkdownInputFile::new(input_file, output_file);

        map.insert("image/svg+xml", Box::new(svg));
        map.insert("image/jpeg", Box::new(jpg));
        map.insert("image/png", Box::new(png));
        map.insert("text/markdown", Box::new(markdown));
        InputsFiles {
            input_file,
            output_file,
            map,
        }
    }

    pub fn support(&self) -> Result<String, Box<dyn Error + 'a>> {
        let input_mime = mime_guess::from_path(self.input_file);
        let e = UnSupportedError {
            input_file: self.input_file,
            output_ext: self.output_file,
        };
        match &input_mime.first_raw() {
            Some(i_mime) => match self.map.get(i_mime) {
                Some(val) => val.support(),
                None => Err(Box::new(e)),
            },
            None => Err(Box::new(e)),
        }
    }

    pub fn mime_map(&self) -> Result<String, Box<dyn Error + 'a>> {
        let input_mime = mime_guess::from_path(self.input_file);
        let e = UnSupportedError {
            input_file: self.input_file,
            output_ext: self.output_file,
        };
        match &input_mime.first_raw() {
            Some(i_mime) => match self.map.get(i_mime) {
                Some(val) => val.mime_map(),
                None => Err(Box::new(e)),
            },
            None => Err(Box::new(e)),
        }
    }
}

trait InputTo<'a> {
    fn convert(&self) -> Result<String, Box<dyn Error + 'a>>;
}

create_input!(SVGInputFile, InputTo);
create_input!(JPGInputFile, InputTo);
create_input!(PNGInputFile, InputTo);
create_input!(MarkdownInputFile, InputTo);
