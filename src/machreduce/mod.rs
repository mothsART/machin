use std::error::Error;

pub mod errors;
pub mod image_reduce;
pub mod zip_reduce;

use crate::machreduce::errors::OutputFileUnsupportedError;

#[derive(PartialEq)]
pub enum Direction {
    Vertical,
    Horizontal
}

pub trait InputTo<'a> {
    fn reduce(&self, direction: &Direction) -> Result<String, Box<dyn Error + 'a>>;
}

#[macro_export]
macro_rules! create_reduce_check_inputs {
    ($struct_name:ident) => {
        pub struct $struct_name<'a> {
            output_file: &'a str,
            input_mime_type: Vec<&'a str>,
            output_mime_type: Vec<&'a str>,
        }
    }
}

#[macro_export]
macro_rules! create_reduce {
    ($struct_name:ident) => {
        pub struct $struct_name<'a> {
            output_file: &'a str,
            output_mime_type: Vec<&'a str>,
        }
    }
}

create_reduce_check_inputs!(ImageOutputFile);
create_reduce!(ZipOutputFile);

pub struct InputsFiles<'a> {
    pub output_file: &'a str,
    pub direction: Direction,
}

impl<'a> InputsFiles<'a> {
    pub fn new(output_file: &'a str, direction: Direction) -> InputsFiles<'a> {
        InputsFiles { output_file, direction }
    }

    pub fn reduce(&mut self) -> Result<String, Box<dyn Error + 'a>> {
        let output_mime = mime_guess::from_path(self.output_file);

        let output_e = OutputFileUnsupportedError {
            output_file: self.output_file,
        };

        let image_output = ImageOutputFile::new(self.output_file);
        let zip_output = ZipOutputFile::new(self.output_file);

        match &output_mime.first_raw() {
            Some(o_mime) => {
                if image_output.output_mime_type.contains(o_mime) {
                    return image_output.reduce(&self.direction);
                }
                if zip_output.output_mime_type.contains(o_mime) {
                    return zip_output.reduce(&self.direction);
                }
                return Err(Box::new(output_e));
            }
            None => {
                return Err(Box::new(output_e));
            }
        };
    }
}
