#[cfg(test)]
mod tests {
    use crypto::digest::Digest;
    use crypto::sha1::Sha1;
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
        hasher.input(&bytes);

        let str_hash = hasher.result_str();
        tmp_dir.close().unwrap();

        return str_hash;
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
            "8788a8a12380265c49a64a89d0ba9cd2a9b24823",
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
}
