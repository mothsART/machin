pub mod pngtoavif;
pub mod pngtojpg;
pub mod pngtopdf;

use crate::machmap::png::pngtoavif::PNGToAVIF;
use crate::machmap::png::pngtojpg::PNGToJPG;
use crate::machmap::png::pngtopdf::PngToPdf;
use crate::machmap::{HashMap, InputTo, PNGInputFile};

impl<'a> PNGInputFile<'a> {
    pub fn new(input_file: &'a str, output_file: &'a str) -> PNGInputFile<'a> {
        let jpg = PNGToJPG::new(input_file, output_file);
        let avif = PNGToAVIF::new(input_file, output_file);
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
