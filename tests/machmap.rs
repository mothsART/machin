#[cfg(test)]
mod tests {
    use std::env;

    use crypto::digest::Digest;
    use crypto::sha1::Sha1;
    use tempfile::tempdir;

    use machin::machmap::InputsFiles;

    fn get_hash_after(input_path: &'static str, output_file: &'static str) -> String {
        let path = env::current_dir().unwrap();
        let input_file = path.join(input_path).display().to_string();

        let tmp_dir = tempdir().unwrap();
        let output_path = tmp_dir.path().join(output_file).display().to_string();

        InputsFiles::new(&input_file, &output_path)
            .mime_map()
            .unwrap();

        let bytes = std::fs::read(&output_path).unwrap();
        let mut hasher = Sha1::new();
        hasher.input(&bytes);

        let str_hash = hasher.result_str();
        tmp_dir.close().unwrap();

        return str_hash;
    }

    #[test]
    fn png_to_jpg() {
        assert_eq!(
            "122267b78644b438a17f99dcb14b37e816554771",
            get_hash_after(
                "tests/datasets/car-vintage-old-rusty.png",
                "car-vintage-old-rusty.jpg"
            )
        );
    }

    #[test]
    fn png_to_pdf() {
        assert_eq!(
            "69eff4d4c989a63036b7a0b7eeebfc164652d456",
            get_hash_after(
                "tests/datasets/car-vintage-old-rusty.png",
                "car-vintage-old-rusty.pdf"
            )
        );
    }

    #[test]
    fn jpg_to_png() {
        assert_eq!(
            "bf6ab61276f04c8093ddcc4db66d5656be3d15af",
            get_hash_after("tests/datasets/rusted_chain.jpg", "rusted_chain.png")
        );
    }

    #[test]
    fn jpg_to_pdf() {
        assert_eq!(
            "7e5be97ebb11d35b04e48484a1d2a0a01483ce31",
            get_hash_after("tests/datasets/rusted_chain.jpg", "rusted_chain.pdf")
        );
    }

    #[test]
    fn svg_to_png() {
        assert_eq!(
            "bd2926601f16f764c569e8c7ee1e8b6f4e106f04",
            get_hash_after(
                "tests/datasets/Rust_programming_language_black_logo.svg",
                "Rust_programming_language_black_logo.png"
            )
        );
    }

    #[test]
    fn svg_to_jpg() {
        assert_eq!(
            "b3f27e39432473af30e0f56b3dacedbfa4fb5ca3",
            get_hash_after(
                "tests/datasets/Rust_programming_language_black_logo.svg",
                "Rust_programming_language_black_logo.jpg"
            )
        );
    }
}
