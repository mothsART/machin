pub mod svg2png;
pub mod svg2jpg;

use crate::mmap::*;
use crate::mmap::svg::svg2jpg::*;
use crate::mmap::svg::svg2png::*;

impl<'a> SVGInputFile<'a> {
    pub fn new(input_file: &'a str, output_file: &'a str) -> SVGInputFile<'a> {
        let png = SVGToPNG::new(input_file, output_file);
        let jpg = SVGToJPG::new(input_file, output_file);

        let mut map: HashMap<&'a str, Box<dyn InputTo<'a> +'a>> = HashMap::new();
        map.insert("image/png", Box::new(png));
        map.insert("image/jpeg", Box::new(jpg));
        SVGInputFile {
            input_file,
            output_file,
            map
        }
    }
}
