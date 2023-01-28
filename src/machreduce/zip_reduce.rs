use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

use colored::*;

use crate::machreduce::{Direction, InputTo};

pub struct ZipOutputFile<'a> {
    pub input_lines: &'a Vec<String>,
    pub output_file: &'a str,
    pub output_mime_type: Vec<&'a str>,
}

impl<'a> ZipOutputFile<'a> {
    pub fn new(input_lines: &'a Vec<String>, output_file: &'a str) -> ZipOutputFile<'a> {
        ZipOutputFile {
            input_lines,
            output_file,
            output_mime_type: vec!["application/zip"],
        }
    }
}

impl<'a> InputTo<'a> for ZipOutputFile<'a> {
    fn reduce(&self, _direction: &Direction) -> Result<String, Box<dyn Error + 'a>> {
        let path = std::path::Path::new(&self.output_file);
        let file = std::fs::File::create(path).unwrap();
        let mut zip = zip::ZipWriter::new(file);
        let options =
            zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);

        for line in self.input_lines {
            let mut buffer = Vec::new();
            if !Path::new(&line).exists() {
                eprintln!(
                    "{}",
                    format!("Input file \"{line}\" doesn't exist")
                        .black()
                        .on_red()
                );
                continue;
            }
            zip.start_file(line, options)?;
            let mut f = File::open(line)?;
            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
        }
        zip.finish()?;
        Ok(format!("images reduce to {}", self.output_file))
    }
}
