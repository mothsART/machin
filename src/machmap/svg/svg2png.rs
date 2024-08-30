use std::error::Error;

use crate::machmap::InputTo;

pub struct SVGToPNG<'a> {
    pub input_file: &'a str,
    pub output_file: &'a str,
}

impl<'a> SVGToPNG<'a> {
    pub fn new(input_file: &'a str, output_file: &'a str) -> SVGToPNG<'a> {
        SVGToPNG {
            input_file,
            output_file,
        }
    }
}

impl<'a> InputTo<'a> for SVGToPNG<'a> {
    fn convert(&self) -> Result<String, Box<dyn Error + 'a>> {
        let opt = usvg::Options::default();

        let svg_data = std::fs::read(self.input_file)?;
        let tree = usvg::Tree::from_data(&svg_data, &opt)?;
        let pixmap_size = tree.size().to_int_size();
        let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
        resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());
        pixmap.save_png(self.output_file)?;
        Ok(format!(
            "convert svg to png : {} -> {}",
            self.input_file, self.output_file
        ))
    }
}
