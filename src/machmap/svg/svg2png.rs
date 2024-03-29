use std::error::Error;

use crate::machmap::InputTo;

pub struct SVGToPNG<'a> {
    pub input_file: &'a str,
    pub output_file: &'a str,
}

impl<'a> SVGToPNG<'a> {
    pub fn new(input_file: &'a str, output_file: &'a str) -> SVGToPNG<'a> {
        SVGToPNG {
            input_file,
            output_file,
        }
    }
}

impl<'a> InputTo<'a> for SVGToPNG<'a> {
    fn convert(&self) -> Result<String, Box<dyn Error + 'a>> {
        let opt = usvg::Options::default();

        let rtree = usvg::Tree::from_file(self.input_file, &opt)?;
        /* TODO : gérer proprement les erreurs tel que :
         * SVG has an invalid size => passer par un autre moteur de rendu ?
         * SVG data parsing failed cause the document does not have a root node :
         * Si le SVG n'a pas de xmlns="http://www.w3.org/2000/svg", le créer à la volée
         */
        let fit_to = usvg::FitTo::Zoom(1.0);
        let pixmap_size = fit_to
            .fit_to(rtree.svg_node().size.to_screen_size())
            .unwrap();
        let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
        resvg::render(&rtree, usvg::FitTo::Original, pixmap.as_mut()).unwrap();
        pixmap.save_png(self.output_file)?;
        Ok(format!(
            "convert svg to png : {} -> {}",
            self.input_file, self.output_file
        ))
    }
}
