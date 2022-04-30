pub mod png2jpg;

use crate::mmap::{InputTo, PNGInputFile, HashMap};
use crate::mmap::png::png2jpg::PNGToJPG;

impl<'a> PNGInputFile<'a> {
    pub fn new(input_file: &'a str, output_file: &'a str) -> PNGInputFile<'a> {
        let jpg = PNGToJPG::new(input_file, output_file);

        let mut map: HashMap<&'a str, Box<dyn InputTo<'a> +'a>> = HashMap::new();
        map.insert("image/jpeg", Box::new(jpg));
        PNGInputFile {
            input_file,
            output_file,
            map
        }
    }
}
