use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::io::Write;

use colored::*;

use crate::machreduce::{InputTo, Direction};
use crate::machreduce::ZipOutputFile;

impl<'a> ZipOutputFile<'a> {
    pub fn new(output_file: &'a str) -> ZipOutputFile<'a> {
        ZipOutputFile {
            output_file,
            output_mime_type: vec!["application/zip"],
        }
    }
}

impl<'a> InputTo<'a> for ZipOutputFile<'a> {
    fn reduce(&self, _direction: &Direction) -> Result<String, Box<dyn Error + 'a>> {
        let lines = std::io::stdin().lines();

        let path = std::path::Path::new(&self.output_file);
        let file = std::fs::File::create(&path).unwrap();
        let mut zip = zip::ZipWriter::new(file);
        let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Stored);
        let mut buffer = Vec::new();

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
                    zip.start_file(&_l, options)?;
                    let mut f = File::open(&_l)?;
                    f.read_to_end(&mut buffer)?;
                    zip.write_all(&*buffer)?;
                }
                Err(_) => {
                    continue;
                }
            }
        }
        zip.finish()?;
        Ok(format!("images reduce to {}", self.output_file))
    }
}
