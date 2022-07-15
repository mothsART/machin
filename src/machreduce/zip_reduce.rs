use std::error::Error;
use std::path::Path;

use colored::*;

use crate::machreduce::InputTo;
use crate::machreduce::ZipOutputFile;

impl<'a> ZipOutputFile<'a> {
    pub fn new(output_file: &'a str) -> ZipOutputFile<'a> {
        ZipOutputFile {
            output_file,
            input_mime_type: vec!["image/png", "image/jpeg"],
            output_mime_type: vec!["image/png", "image/jpeg"],
        }
    }
}

impl<'a> InputTo<'a> for ZipOutputFile<'a> {
    fn reduce(&self) -> Result<String, Box<dyn Error + 'a>> {
        let lines = std::io::stdin().lines();

        for line in lines {
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
                    continue;
                }
                Err(_) => {
                    continue;
                }
            }
        }

        Ok(format!("images reduce to {}", self.output_file))
    }
}
