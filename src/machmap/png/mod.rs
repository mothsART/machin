pub mod pngtopdf;

use image::io::Reader as ImageReader;

use crate::machmap::png::pngtopdf::PngToPdf;
use crate::machmap::{Error, HashMap, InputTo, PNGInputFile};

impl<'a> PNGInputFile<'a> {
    pub fn new(input_file: &'a str, output_file: &'a str) -> PNGInputFile<'a> {
        convert_img!(PngToJpg, "png", "jpg");
        let jpg = PngToJpg::new(input_file, output_file);

        convert_img!(PngToAvif, "png", "avif");
        let avif = PngToAvif::new(input_file, output_file);

        let pdf = PngToPdf::new(input_file, output_file);

        let mut map: HashMap<&'a str, Box<dyn InputTo<'a> + 'a>> = HashMap::new();
        map.insert("image/jpeg", Box::new(jpg));
        map.insert("image/avif", Box::new(avif));
        map.insert("application/pdf", Box::new(pdf));
        PNGInputFile {
            input_file,
            output_file,
            map,
        }
    }
}
