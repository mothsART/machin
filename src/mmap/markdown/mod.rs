impl<'a> MarkdownInputFile<'a> {
    fn new(input_file: &'a str, output_file: &'a str) -> MarkdownInputFile<'a> {
        let png = SVGToPNG::new(input_file, output_file);
        let jpg = SVGToJPG::new(input_file, output_file);

        let mut map: HashMap<&'a str, Box<dyn SVGTo<'a> +'a>> = HashMap::new();
        map.insert("image/png", Box::new(png));
        map.insert("image/jpeg", Box::new(jpg));
        MarkdownInputFile {
            input_file: input_file,
            output_file: output_file,
            map: map
        }
    }
}
