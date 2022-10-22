#[cfg(test)]
mod tests {
    use std::env;

    use crypto::digest::Digest;
    use crypto::sha1::Sha1;
    use tempfile::tempdir;

    use machin::machmap::InputsFiles;

    #[test]
    fn jpg_to_png() {
        let path = env::current_dir().unwrap();
        let input_file = path
            .join("tests/datasets/rusted_chain.jpg")
            .display()
            .to_string();

        let tmp_dir = tempdir().unwrap();
        let output_png_file = tmp_dir
            .path()
            .join("rusted_chain.png")
            .display()
            .to_string();

        InputsFiles::new(&input_file, &output_png_file)
            .mime_map()
            .unwrap();

        let bytes = std::fs::read(&output_png_file).unwrap();
        let mut hasher = Sha1::new();
        hasher.input(&bytes);

        assert_eq!(
            "bf6ab61276f04c8093ddcc4db66d5656be3d15af",
            &hasher.result_str()
        );
        tmp_dir.close().unwrap();
    }
}
