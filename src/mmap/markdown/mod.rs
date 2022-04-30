use crate::mmap::*;

impl<'a> MarkdownInputFile<'a> {
    fn new(input_file: &'a str, output_file: &'a str) -> MarkdownInputFile<'a> {
        //let png = SVGToPNG::new(input_file, output_file);
        //let jpg = SVGToJPG::new(input_file, output_file);

        let map: HashMap<&'a str, Box<dyn InputTo<'a> + 'a>> = HashMap::new();
        //map.insert("image/png", Box::new(png));
        //map.insert("image/jpeg", Box::new(jpg));
        MarkdownInputFile {
            input_file,
            output_file,
            map,
        }
    }
}
