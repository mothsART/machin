pub mod svg2avif;
pub mod svg2jpg;
pub mod svg2pdf;
pub mod svg2png;

use crate::machmap::svg::svg2avif::SVGToAVIF;
use crate::machmap::svg::svg2jpg::SVGToJPG;
use crate::machmap::svg::svg2pdf::SVGToPDF;
use crate::machmap::svg::svg2png::SVGToPNG;
use crate::machmap::*;

impl<'a> SVGInputFile<'a> {
    pub fn new(input_file: &'a str, output_file: &'a str) -> SVGInputFile<'a> {
        let jpg = SVGToJPG::new(input_file, output_file);
        let avif = SVGToAVIF::new(input_file, output_file);
        let pdf = SVGToPDF::new(input_file, output_file);
        let png = SVGToPNG::new(input_file, output_file);

        let mut map: HashMap<&'a str, Box<dyn InputTo<'a> + 'a>> = HashMap::new();
        map.insert("image/png", Box::new(png));
        map.insert("image/avif", Box::new(avif));
        map.insert("image/jpeg", Box::new(jpg));
        map.insert("application/pdf", Box::new(pdf));
        SVGInputFile {
            input_file,
            output_file,
            map,
        }
    }
}
