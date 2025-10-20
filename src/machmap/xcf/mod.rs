use crate::machmap::{xcf::xcf2png::XcfToPng, HashMap, InputTo, XcfInputFile};
pub mod xcf2png;


impl<'a> XcfInputFile<'a> {
    pub fn new(input_file: &'a str, output_file: &'a str) -> XcfInputFile<'a> {
        let png = XcfToPng::new(input_file, output_file);

        let mut map: HashMap<&'a str, Box<dyn InputTo<'a> + 'a>> = HashMap::new();
        map.insert("image/png", Box::new(png));

        XcfInputFile {
            input_file,
            output_file,
            map,
        }
    }
}
