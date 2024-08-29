use image::ImageReader;

use crate::machmap::{Error, HashMap, InputTo, WebpInputFile};

impl<'a> WebpInputFile<'a> {
    pub fn new(input_file: &'a str, output_file: &'a str) -> WebpInputFile<'a> {
        convert_img!(WebpToPng, "webp", "png");
        let png = WebpToPng::new(input_file, output_file);

        convert_img!(WebpToJpg, "webp", "jpg");
        let jpg = WebpToJpg::new(input_file, output_file);

        let mut map: HashMap<&'a str, Box<dyn InputTo<'a> + 'a>> = HashMap::new();
        map.insert("image/png", Box::new(png));
        map.insert("image/jpeg", Box::new(jpg));
        WebpInputFile {
            input_file,
            output_file,
            map,
        }
    }
}
