use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

use map_macro::hash_map;
use sha1::{Digest, Sha1};

use machin::machmap::InputsFiles;

fn get_hash_after(input_path: &'static str, output_file: &'static str) -> String {
    let path = env::current_dir().unwrap();
    let input_file = path.join(input_path).display().to_string();
    let output_path = path.join(output_file).display().to_string();

    InputsFiles::new(&input_file, &output_path)
        .mime_map()
        .unwrap();

    let bytes = std::fs::read(&output_path).unwrap();
    let mut hasher = Sha1::new();
    hasher.update(&bytes);

    let str_hash = format!("{:x}", hasher.finalize());
    return str_hash;
}

fn get_zip_hashes(input_file: &'static str, output_file: &'static str) -> HashMap<String, String> {
    let mut hashes = HashMap::new();

    let path = env::current_dir().unwrap();
    let output_path = path.join(output_file).display().to_string();

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

    return hashes;
}

#[test]
fn jpg_to_odt() {
    assert_eq!(
        hash_map! {
            "content.xml".to_string() => "77853510e108e143a7e8c8b29f5413f37771d9ee".to_string(),
            "Pictures/rusted_chain.jpg".to_string() => "7dcab112baadeb6c58b2091b84b03421ad1e44ea".to_string(),
            "META-INF/manifest.xml".to_string() => "4b302bb7954931a255ecd4d765c6c62396613b68".to_string()
        },
        get_zip_hashes(
            "tests/datasets/rusted_chain.jpg",
            "tests/datasets/machmap/rusted_chain.odt"
        )
    );
}

#[test]
fn svg_to_png() {
    assert_eq!(
        "9dfb6b219ced6ff116df3d7aed39a8acd94db505",
        get_hash_after(
            "tests/datasets/Rust_programming_language_black_logo.svg",
            "tests/datasets/machmap/Rust_programming_language_black_logo.png"
        )
    );
}

#[test]
fn svg_to_jpg() {
    assert_eq!(
        "a655c1cd185f924145a349cc83b8b67bdc656a24",
        get_hash_after(
            "tests/datasets/Rust_programming_language_black_logo.svg",
            "tests/datasets/machmap/Rust_programming_language_black_logo.jpg"
        )
    );
}

#[test]
fn svg_to_avif() {
    assert_eq!(
        "06e2258e37b3f2a0852e701e0deda41f3a181a0f",
        get_hash_after(
            "tests/datasets/Rust_programming_language_black_logo.svg",
            "tests/datasets/machmap/Rust_programming_language_black_logo.avif"
        )
    );
}

#[test]
fn svg_to_pdf() {
    let hash = get_hash_after(
        "tests/datasets/Rust_programming_language_black_logo.svg",
        "tests/datasets/machmap/Rust_programming_language_black_logo.pdf"
    );
    if hash == "4efbfd4160b8c78f8a6ac72ff742f3a89b076d4c" || hash == "3a7f5098643f33ccbcd70b08da50adac8a69717c" {
        assert!(true);
    } else {
        assert!(false);
    }
}

#[test]
fn webp_to_jpg() {
    assert_eq!(
        "50db5f0fcf0ff48ad2bf73f66d9aab48aff2438d",
        get_hash_after(
            "tests/datasets/house.webp",
            "tests/datasets/machmap/house.jpg"
        )
    );
}

#[test]
fn webp_to_png() {
    assert_eq!(
        "c594c0c7e329a657c6b6cb074c90a185aabbf238",
        get_hash_after(
            "tests/datasets/house.webp",
            "tests/datasets/machmap/house.png"
        )
    );
}

#[test]
fn webp_to_avif() {
    assert_eq!(
        "ded9ac7d150e74b53037a7c63e79e5dc11a71e3f",
        get_hash_after(
            "tests/datasets/house.webp",
            "tests/datasets/machmap/house.avif"
        )
    );
}

#[test]
fn png_to_jpg() {
    assert_eq!(
        "e98cb15b9b71e4e43c017276d17b0df0bba3d2db",
        get_hash_after(
            "tests/datasets/car-vintage-old-rusty__with_alpha.png",
            "tests/datasets/machmap/car-vintage-old-rusty__with_alpha.jpg"
        )
    );
    assert_eq!(
        "122267b78644b438a17f99dcb14b37e816554771",
        get_hash_after(
            "tests/datasets/car-vintage-old-rusty__without_alpha.png",
            "tests/datasets/machmap/car-vintage-old-rusty__without_alpha.jpg"
        )
    );
}

#[test]
fn png_to_avif() {
    assert_eq!(
        "5a57f1c897682401ae08869d43de00c8a3ccb097",
        get_hash_after(
            "tests/datasets/car-vintage-old-rusty__with_alpha.png",
            "tests/datasets/machmap/car-vintage-old-rusty__with_alpha.avif"
        )
    );
    assert_eq!(
        "45acf0803334fb2d71b1a5086dc6af24c922ac10",
        get_hash_after(
            "tests/datasets/car-vintage-old-rusty__without_alpha.png",
            "tests/datasets/machmap/car-vintage-old-rusty__without_alpha.avif"
        )
    );
}

#[test]
fn png_to_pdf() {
    assert_eq!(
        "97852a8bdf45c4926e849c68ac1557b34aa49733",
        get_hash_after(
            "tests/datasets/car-vintage-old-rusty__with_alpha.png",
            "tests/datasets/machmap/car-vintage-old-rusty__with_alpha.pdf"
        )
    );
    assert_eq!(
        "1f18ea64a2c81a55c56c0dbb6d71c6d2e2da0206",
        get_hash_after(
            "tests/datasets/car-vintage-old-rusty__without_alpha.png",
            "tests/datasets/machmap/car-vintage-old-rusty__without_alpha.pdf"
        )
    );
}

#[test]
fn jpg_to_png() {
    assert_eq!(
        "6177356ecb2f23b6d8ce304b559bc56f5d82e188",
        get_hash_after(
            "tests/datasets/rusted_chain.jpg",
            "tests/datasets/machmap/rusted_chain.png"
        )
    );
}

#[test]
fn jpg_to_avif() {
    assert_eq!(
        "265f6e80f0caef933048b8ccf09bdd3cef3aa060",
        get_hash_after(
            "tests/datasets/rusted_chain.jpg",
            "tests/datasets/machmap/rusted_chain.avif"
        )
    );
}

#[test]
fn jpg_to_pdf() {
    assert_eq!(
        "53f5a3a3f2c8f673fceb3e120a80994ff474a262",
        get_hash_after(
            "tests/datasets/rusted_chain.jpg",
            "tests/datasets/machmap/rusted_chain.pdf"
        )
    );
}

#[test]
fn md_to_html() {
    assert_eq!(
        "683817dd0cdc0b63a26c8a58cc05f9f31f26ad5f",
        get_hash_after(
            "tests/datasets/markdown.md",
            "tests/datasets/machmap/markdown.html"
        )
    );
}

#[test]
fn json_to_yaml() {
    assert_eq!(
        "a15505250ffdefc95cd6ec0bbb914b196e96e3e9",
        get_hash_after(
            "tests/datasets/example.json",
            "tests/datasets/machmap/example.yaml"
        )
    );
}

#[test]
fn yaml_to_json() {
    assert_eq!(
        "548dbfa8e473fc9453df1518c39d11f74fff6af1",
        get_hash_after(
            "tests/datasets/docker-compose.yaml",
            "tests/datasets/machmap/docker-compose.json"
        )
    );
}
