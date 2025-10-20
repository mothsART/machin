use std::error::Error;

use xcf_rs::data::xcf::Xcf;

use crate::machmap::InputTo;

pub struct XcfToPng<'a> {
    pub input_file: &'a str,
    pub output_file: &'a str,
}

impl<'a> XcfToPng<'a> {
    pub fn new(input_file: &'a str, output_file: &'a str) -> XcfToPng<'a> {
        XcfToPng {
            input_file,
            output_file,
        }
    }
}

impl<'a> InputTo<'a> for XcfToPng<'a> {
    fn convert(&self) -> Result<String, Box<dyn Error + 'a>> {
        let  xcf = Xcf::open(self.input_file)?;
        let pixels = xcf.layers[0].pixel(0, 0);
        println!("pixels: {:?}", pixels);
        Ok("prout".to_string())
    }
}
