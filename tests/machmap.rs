#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::env;
    use std::fs::File;
    use std::io::Read;

    use map_macro::map;
    use sha1::{Digest, Sha1};
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
        hasher.update(&bytes);

        let str_hash = format!("{:x}", hasher.finalize());
        tmp_dir.close().unwrap();

        return str_hash;
    }

    fn get_zip_hashes(
        input_file: &'static str,
        output_file: &'static str,
    ) -> HashMap<String, String> {
        let mut hashes = HashMap::new();

        let tmp_dir = tempdir().unwrap();
        let output_path = tmp_dir.path().join(output_file).display().to_string();

        InputsFiles::new(&input_file, &output_path)
            .mime_map()
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

            let file_name = file.name();
            hashes.insert(file_name.to_string(), str_hash);
        }

        tmp_dir.close().unwrap();
        return hashes;
    }

    #[test]
    fn jpg_to_odt() {
        assert_eq!(
            map! {
                "content.xml".to_string() => "77853510e108e143a7e8c8b29f5413f37771d9ee".to_string(),
                "Pictures/rusted_chain.jpg".to_string() => "312ca494310f40c465fb0de587d90580566e969a".to_string(),
                "META-INF/manifest.xml".to_string() => "4b302bb7954931a255ecd4d765c6c62396613b68".to_string()
            },
            get_zip_hashes("tests/datasets/rusted_chain.jpg", "rusted_chain.odt")
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

    #[test]
    fn webp_to_jpg() {
        assert_eq!(
            "3077294abcd351689f5e261f940eddd9e2970dd0",
            get_hash_after("tests/datasets/house.webp", "house.jpg")
        );
    }

    #[test]
    fn webp_to_png() {
        assert_eq!(
            "0f848e7f44014c6f9ce82707c13b386fe94f336c",
            get_hash_after("tests/datasets/house.webp", "house.png")
        );
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
            "422fff296caf52cc6086d1e2d485b16e06daa2f5",
            get_hash_after(
                "tests/datasets/car-vintage-old-rusty.png",
                "car-vintage-old-rusty.pdf"
            )
        );
    }

    #[test]
    fn jpg_to_png() {
        assert_eq!(
            "2edbffe70d5b9a61b2c1545d98cb6f05216b33b5",
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
    fn md_to_html() {
        assert_eq!(
            "683817dd0cdc0b63a26c8a58cc05f9f31f26ad5f",
            get_hash_after("tests/datasets/markdown.md", "markdown.html")
        );
    }

    #[test]
    fn json_to_yaml() {
        assert_eq!(
            "a15505250ffdefc95cd6ec0bbb914b196e96e3e9",
            get_hash_after("tests/datasets/example.json", "example.yaml")
        );
    }

    #[test]
    fn yaml_to_json() {
        assert_eq!(
            "548dbfa8e473fc9453df1518c39d11f74fff6af1",
            get_hash_after("tests/datasets/docker-compose.yaml", "docker-compose.json")
        );
    }
}
