use std::error::Error;
use std::path::PathBuf;

use crate::machmap::InputTo;

use image::ImageReader;
use image::GenericImageView;

use xcf_rs::create::XcfCreator;
use xcf_rs::data::color::ColorType;
use xcf_rs::data::rgba::RgbaPixel;
use xcf_rs::data::pixeldata::PixelData;
use xcf_rs::data::layer::Layer;
use xcf_rs::{LayerColorType, LayerColorValue};

pub struct JpgToXcf<'a> {
    pub input_file: &'a str,
    pub output_file: &'a str,
}

impl<'a> JpgToXcf<'a> {
    pub fn new(input_file: &'a str, output_file: &'a str) -> JpgToXcf<'a> {
        JpgToXcf {
            input_file,
            output_file,
        }
    }
}

impl<'a> InputTo<'a> for JpgToXcf<'a> {
    fn convert(&self) -> Result<String, Box<dyn Error + 'a>> {
        let img = ImageReader::open(self.input_file)?.decode()?;
        let dimensions = img.dimensions();
        let width = dimensions.0;
        let height = dimensions.1;


        let mut xcf = XcfCreator::new(11, width, height, ColorType::Rgb);
        xcf.add_properties(&vec![]);
    
        let mut layers = vec![];
        let mut pixels = vec![];

        for p in img.pixels() {
            let rgba = p.2.0;
            pixels.push(RgbaPixel::new(
                *rgba.get(0).unwrap(),
                *rgba.get(1).unwrap(), 
                *rgba.get(2).unwrap(), 
                *rgba.get(3).unwrap()
            ));
        }
        
        let pixels_layer_one: PixelData = PixelData {
            width: width,
            height: height,
            pixels: pixels,
        };
        let properties_layer_one = vec![];
        let mut kind = LayerColorType {
            kind: LayerColorValue::Rgb,
            alpha: false,
        };
        if img.color().has_alpha() {
            kind = LayerColorType {
                kind: LayerColorValue::Rgba,
                alpha: true,
            };
        }
        let layer_one = Layer {
            width,
            height,
            kind,
            name: "Background".to_string(),
            pixels: pixels_layer_one,
            properties: properties_layer_one,
        };
        layers.push(layer_one);
        xcf.add_layers(&layers);
        let output_path = PathBuf::from(self.output_file);
        xcf.save(&output_path)?;
        Ok("".to_string())
    }
}