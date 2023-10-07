#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;

    use sha1::{Sha1, Digest};
    use map_macro::map;
    use tempfile::tempdir;

    use machin::machreduce::{Direction, InputsFiles};

    fn get_hash_after(
        inputs_path: Vec<&'static str>,
        output_file: &'static str,
        direction: Direction,
    ) -> String {
        let mut inputs_file = vec![];
        for input_path in inputs_path {
            inputs_file.push(input_path.to_string());
        }

        let tmp_dir = tempdir().unwrap();
        let output_path = tmp_dir.path().join(output_file).display().to_string();

        InputsFiles::new(&inputs_file, &output_path, direction)
            .reduce()
            .unwrap();

        let bytes = std::fs::read(&output_path).unwrap();
        let mut hasher = Sha1::new();
        hasher.update(&bytes);

        let str_hash = format!("{:x}", hasher.finalize());
        tmp_dir.close().unwrap();

        return str_hash;
    }

    #[test]
    fn create_zip() {
        let mut inputs_file = vec![];

        let hashes = map! {
          "tests/datasets/rusted_gears.jpg" => "b6f1c03e2893f8afd5dfa5d3ac6cabb67222fd21",
          "tests/datasets/rusted_chain.jpg" => "312ca494310f40c465fb0de587d90580566e969a",
          "tests/datasets/car-vintage-old-rusty.png" => "f474d4b8629ff0d34296b9f7c825c020029b92ac",
        };

        for h in hashes.keys() {
            inputs_file.push(h.to_string());
        }

        let tmp_dir = tempdir().unwrap();
        let output_path = tmp_dir.path().join("result.zip").display().to_string();

        InputsFiles::new(&inputs_file, &output_path, Direction::Vertical)
            .reduce()
            .unwrap();

        let zip_file = File::open(&output_path).unwrap();
        let mut zip = zip::ZipArchive::new(zip_file).unwrap();

        for i in 0..zip.len() {
            let mut file = zip.by_index(i).unwrap();
            let mut hasher = Sha1::new();
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).unwrap();
            hasher.update(&buffer);
            let str_hash = format!("{:x}", hasher.finalize());
            assert_eq!(hashes.get(file.name()).unwrap(), &str_hash);
        }

        tmp_dir.close().unwrap();
    }

    #[test]
    fn create_vertical_jpg() {
        assert_eq!(
            "a4768df3082baaf6879b2356a701c25b8807fc85",
            get_hash_after(
                vec![
                    "tests/datasets/rusted_chain.jpg",
                    "tests/datasets/rusted_gears.jpg",
                ],
                "result.jpg",
                Direction::Vertical
            )
        );
    }

    #[test]
    fn create_horizontal_jpg() {
        assert_eq!(
            "030da623bf7b81d28a80caa2a638dcaca5bd443c",
            get_hash_after(
                vec![
                    "tests/datasets/rusted_chain.jpg",
                    "tests/datasets/rusted_gears.jpg",
                ],
                "result.jpg",
                Direction::Horizontal
            )
        );
    }

    #[test]
    fn create_png_with_heterogeneous_pictures() {
        assert_eq!(
            "94288c4505baf2306c36c4930ac8bd44f2b65e82",
            get_hash_after(
                vec![
                    "tests/datasets/rusted_gears.jpg",
                    "tests/datasets/car-vintage-old-rusty.png",
                ],
                "result.png",
                Direction::Vertical
            )
        );
    }

    #[test]
    fn create_pdf_with_heterogeneous_pictures() {
        assert_eq!(
            "1c59ce7b5063e9d452a2645ec1276ff7f390eded",
            get_hash_after(
                vec![
                    "tests/datasets/rusted_gears.jpg",
                    "tests/datasets/car-vintage-old-rusty.png",
                ],
                "result.pdf",
                Direction::Vertical
            )
        );
    }

    #[test]
    fn create_pdf_with_multiple_pdf_files() {
        assert_eq!(
            "cbfae5ac87499cf157b5073a6726db5c191bf103",
            get_hash_after(
                vec![
                    "tests/datasets/first_doc.pdf",
                    "tests/datasets/second_doc.pdf",
                ],
                "merge.pdf",
                Direction::Vertical
            )
        );
    }
}
