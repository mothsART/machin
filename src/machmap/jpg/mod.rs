pub mod jpgtoodt;
pub mod jpgtopdf;

use image::ImageReader;

use crate::machmap::jpg::jpgtoodt::JpgToOdt;
use crate::machmap::jpg::jpgtopdf::JpgToPdf;
use crate::machmap::{Error, HashMap, InputTo, JPGInputFile};

impl<'a> JPGInputFile<'a> {
    pub fn new(input_file: &'a str, output_file: &'a str) -> JPGInputFile<'a> {
        convert_img!(JpgToPng, "jpg", "png");
        let png = JpgToPng::new(input_file, output_file);

        //convert_img!(JpgToAvif, "jpg", "avif");
        //let avif = JpgToAvif::new(input_file, output_file);

        let pdf = JpgToPdf::new(input_file, output_file);
        let odt = JpgToOdt::new(input_file, output_file);

        let mut map: HashMap<&'a str, Box<dyn InputTo<'a> + 'a>> = HashMap::new();
        map.insert("image/png", Box::new(png));
        //map.insert("image/avif", Box::new(avif));
        map.insert("application/pdf", Box::new(pdf));
        map.insert("application/vnd.oasis.opendocument.text", Box::new(odt));
        JPGInputFile {
            input_file,
            output_file,
            map,
        }
    }
}
