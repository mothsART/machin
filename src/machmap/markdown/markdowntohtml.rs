extern crate markdown;

use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::machmap::InputTo;

pub struct MarkdownToHTML<'a> {
    pub input_file: &'a str,
    pub output_file: &'a str,
}

impl<'a> MarkdownToHTML<'a> {
    pub fn new(input_file: &'a str, output_file: &'a str) -> MarkdownToHTML<'a> {
        MarkdownToHTML {
            input_file,
            output_file,
        }
    }
}

impl<'a> InputTo<'a> for MarkdownToHTML<'a> {
    fn convert(&self) -> Result<String, Box<dyn Error + 'a>> {
        let html = markdown::file_to_html(Path::new(&self.input_file));
        let mut file = File::create(self.output_file)?;
        file.write_all(html?.as_bytes())?;
        Ok(format!(
            "convert markdown to html : {} -> {}",
            self.input_file, self.output_file
        ))
    }
}
