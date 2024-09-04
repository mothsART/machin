use std::error::Error;

use svg2pdf::{ConversionOptions, PageOptions};

use crate::machmap::InputTo;

pub struct SVGToPDF<'a> {
    pub input_file: &'a str,
    pub output_file: &'a str,
}

impl<'a> SVGToPDF<'a> {
    pub fn new(input_file: &'a str, output_file: &'a str) -> SVGToPDF<'a> {
        SVGToPDF {
            input_file,
            output_file,
        }
    }
}

impl<'a> InputTo<'a> for SVGToPDF<'a> {
    fn convert(&self) -> Result<String, Box<dyn Error + 'a>> {
        let svg = std::fs::read_to_string(self.input_file)?;
        let mut options = svg2pdf::usvg::Options::default();
        options.fontdb_mut().load_system_fonts();
        let tree = svg2pdf::usvg::Tree::from_str(&svg, &options)?;

        let pdf = svg2pdf::to_pdf(&tree, ConversionOptions::default(), PageOptions::default());
        std::fs::write(self.output_file, pdf)?;
        Ok(format!(
            "convert svg to pdf : {} -> {}",
            self.input_file, self.output_file
        ))
    }
}
