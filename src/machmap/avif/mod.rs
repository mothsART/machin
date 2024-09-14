use image::{ImageFormat, ImageReader};
use colored::Colorize;

use crate::machmap::{Error, HashMap, InputTo, AVIFInputFile};

impl<'a> AVIFInputFile<'a> {
    pub fn new(input_file: &'a str, output_file: &'a str) -> AVIFInputFile<'a> {
        convert_img!(AvifToPng, "avif", "png");
        let png = AvifToPng::new(input_file, output_file);

        let mut map: HashMap<&'a str, Box<dyn InputTo<'a> + 'a>> = HashMap::new();
        map.insert("image/png", Box::new(png));
        AVIFInputFile {
            input_file,
            output_file,
            map,
        }
    }
}
