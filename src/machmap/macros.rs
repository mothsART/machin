macro_rules! create_input {
    ($struct_name:ident, $convert_trait:ident) => {
        struct $struct_name<'a> {
            input_file: &'a str,
            output_file: &'a str,
            map: HashMap<&'a str, Box<dyn $convert_trait<'a> + 'a>>,
        }

        impl<'a> IFile<'a> for $struct_name<'a> {
            fn support(&self) -> Result<String, Box<dyn Error + 'a>> {
                let mut result = "".to_string();
                for (key, _) in &self.map {
                    result = format!("{}{}\n", result, key);
                }
                Ok(result)
            }

            fn mime_map(&self) -> Result<String, Box<dyn Error + 'a>> {
                let output_mime = mime_guess::from_path(self.output_file);
                let e = UnSupportedError {
                    input_file: self.input_file,
                    output_ext: self.output_file,
                };
                match &output_mime.first_raw() {
                    Some(i_mime) => match self.map.get(i_mime) {
                        Some(val) => val.convert(),
                        None => Err(Box::new(e)),
                    },
                    None => Err(Box::new(e)),
                }
            }
        }
    };
}

macro_rules! convert_img {
    (
        $struct_name:ident,
        $input_name:expr,
        $output_name:expr
    ) => {
        struct $struct_name<'a> {
            input_file: &'a str,
            output_file: &'a str,
        }

        impl<'a> $struct_name<'a> {
            pub fn new(input_file: &'a str, output_file: &'a str) -> $struct_name<'a> {
                $struct_name {
                    input_file,
                    output_file,
                }
            }
        }

        impl<'a> InputTo<'a> for $struct_name<'a> {
            fn convert(&self) -> Result<String, Box<dyn Error + 'a>> {
                println!("before open avif");
                let dyn_img = ImageReader::open(&self.input_file)?;
                let format = ImageFormat::from_path(&self.input_file);
                if format == Some(Avif) {
                    let data = std::fs::read(&self.input_file)?;
                    let d = Decoder::from_avif(&data)?;
                    let encoded = match d.to_image()? {
                        Image::Rgb8(img) => {
                            let (buf, width, height) = img.into_contiguous_buf();
                            lodepng::encode_memory(&buf, width, height, lodepng::ColorType::RGB, 8)
                        },
                        _ => {}
                    }?;
                    std::fs::write(&eslf.output_file, encoded);
                    Ok(format!(
                        "convert {} to {} : {} -> {}",
                        $input_name, $output_name, self.input_file, self.output_file,
                    ))
                }

                println!("after open avif {:?}", ImageFormat::from_path(&self.input_file).ok());
                let img = dyn_img.decode()?;
                println!("after decode avif");
                let format = ImageFormat::from_path(self.output_file)?;
                if format == ImageFormat::Jpeg {
                    if img.color().has_alpha() {
                        colored_warn!(format!(
                            "Warning : input file \"{}\" have an alpha channel : is not supported for en jpeg file. The output file \"{}\" will no longer have an alpha channel.",
                            self.input_file,
                            self.output_file,
                        ));
                    }
                    img.to_rgb8().save(self.output_file)?;
                } else {
                    img.save(&self.output_file)?;
                }
                Ok(format!(
                    "convert {} to {} : {} -> {}",
                    $input_name, $output_name, self.input_file, self.output_file,
                ))
            }
        }
    };
}
