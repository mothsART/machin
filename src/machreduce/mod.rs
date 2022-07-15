use std::error::Error;

pub mod image_reduce;
pub mod errors;

use crate::machreduce::image_reduce::InputTo;
use crate::machreduce::errors::OutputFileUnsupportedError;

#[macro_export]
macro_rules! create_reduce_input {
    ($struct_name:ident) => {
        pub struct $struct_name<'a> {
            output_file: &'a str,
            input_mime_type: Vec<&'a str>,
            output_mime_type: Vec<&'a str>,
        }
    };
}

create_reduce_input!(ImageInputFile);

pub struct InputsFiles<'a> {
    pub output_file: &'a str
}

impl<'a> InputsFiles<'a> {
    pub fn new(output_file: &'a str) -> InputsFiles<'a> {
        InputsFiles {
            output_file
        }
    }

    pub fn reduce(&mut self) -> Result<String, Box<dyn Error + 'a>> {
        let output_mime = mime_guess::from_path(self.output_file);

        let input_e = OutputFileUnsupportedError {
            output_file: self.output_file,
        };

        let image_input = ImageInputFile::new(self.output_file);

        match &output_mime.first_raw() {
            Some(o_mime) => {
                println!("{:?}", o_mime);
                if image_input.output_mime_type.contains(o_mime) {
                    return image_input.reduce();
                }
                return Err(Box::new(input_e));
            }
            None => {
                return Err(Box::new(input_e));
            }
        };
    }
}
