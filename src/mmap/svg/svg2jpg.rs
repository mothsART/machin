use std::error::Error;

use crate::mmap::InputTo;

pub struct SVGToJPG<'a> {
    pub input_file: &'a str,
    pub output_file: &'a str
}

impl<'a> SVGToJPG<'a> {
    pub fn new(input_file: &'a str, output_file: &'a str) -> SVGToJPG<'a> {
        SVGToJPG {
            input_file: input_file,
            output_file: output_file
        }
    }
}

impl<'a> InputTo<'a> for SVGToJPG<'a> {
    fn convert(&self) -> Result<(), Box<dyn Error + 'a>> {
        println!("convert svg to jpg");
        Ok(())
    }
}
