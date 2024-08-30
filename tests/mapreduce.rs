use std::env;
use std::fs::File;
use std::io::Read;

use map_macro::hash_map;
use sha1::{Digest, Sha1};

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

    let path = env::current_dir().unwrap();
    let output_path = path.join(output_file).display().to_string();

    InputsFiles::new(&inputs_file, &output_path, direction)
        .reduce()
        .unwrap();

    let bytes = std::fs::read(&output_path).unwrap();
    let mut hasher = Sha1::new();
    hasher.update(&bytes);

    let str_hash = format!("{:x}", hasher.finalize());

    return str_hash;
}

#[test]
fn create_zip() {
    let mut inputs_file = vec![];

    let hashes = hash_map! {
        "tests/datasets/rusted_gears.jpg" => "b6f1c03e2893f8afd5dfa5d3ac6cabb67222fd21",
        "tests/datasets/rusted_chain.jpg" => "7dcab112baadeb6c58b2091b84b03421ad1e44ea",
        "tests/datasets/car-vintage-old-rusty__with_alpha.png" => "1baeb1db563b4c405d3da2b8039fbeebb9cedb7e",
    };

    for h in hashes.keys() {
        inputs_file.push(h.to_string());
    }

    let path = env::current_dir().unwrap();
    let output_path = path
        .join("tests/datasets/mapreduce/result.zip")
        .display()
        .to_string();

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
}

#[test]
fn create_vertical_jpg() {
    assert_eq!(
        "386da50e983fc714b1e2211e978c9d3ea8da8d5e",
        get_hash_after(
            vec![
                "tests/datasets/rusted_chain.jpg",
                "tests/datasets/rusted_gears.jpg",
            ],
            "tests/datasets/mapreduce/rusted_chain__vertical__rusted_gears.jpg",
            Direction::Vertical
        )
    );
}

#[test]
fn create_horizontal_jpg() {
    assert_eq!(
        "3e1838bdf4eecb39dbdf13e7d39bdf897c4847f3",
        get_hash_after(
            vec![
                "tests/datasets/rusted_chain.jpg",
                "tests/datasets/rusted_gears.jpg",
            ],
            "tests/datasets/mapreduce/rusted_chain__horizontal__rusted_gears.jpg",
            Direction::Horizontal
        )
    );
}

#[test]
fn create_png_with_heterogeneous_pictures() {
    assert_eq!(
        "db7d2226668fbe6bb9be5ee49e73ae671a57f21a",
        get_hash_after(
            vec![
                "tests/datasets/rusted_gears.jpg",
                "tests/datasets/car-vintage-old-rusty__with_alpha.png",
            ],
            "tests/datasets/mapreduce/with_heterogeneous_pictures_with_alpha.png",
            Direction::Vertical
        )
    );
    assert_eq!(
        "7864e925834b66598d4cbbce719d6b55e35828a4",
        get_hash_after(
            vec![
                "tests/datasets/rusted_gears.jpg",
                "tests/datasets/car-vintage-old-rusty__without_alpha.png",
            ],
            "tests/datasets/mapreduce/with_heterogeneous_pictures__without_alpha.png",
            Direction::Vertical
        )
    );
}

#[test]
fn create_pdf_with_heterogeneous_pictures() {
    assert_eq!(
        "de33e6c72045eb153e7786554be8307afe657ba9",
        get_hash_after(
            vec![
                "tests/datasets/rusted_gears.jpg",
                "tests/datasets/car-vintage-old-rusty__with_alpha.png",
            ],
            "tests/datasets/mapreduce/with_heterogeneous_pictures__with_alpha.pdf",
            Direction::Vertical
        )
    );
    assert_eq!(
        "1d3abf3b92f15d086d02c040e9b008ec844ed5e2",
        get_hash_after(
            vec![
                "tests/datasets/rusted_gears.jpg",
                "tests/datasets/car-vintage-old-rusty__without_alpha.png",
            ],
            "tests/datasets/mapreduce/with_heterogeneous_pictures__without_alpha.pdf",
            Direction::Vertical
        )
    );
}

#[test]
fn create_pdf_with_multiple_pdf_files() {
    assert_eq!(
        "b5e477c7a010b801aa25d3a6051619ec6b242b8f",
        get_hash_after(
            vec![
                "tests/datasets/first_doc.pdf",
                "tests/datasets/second_doc.pdf",
            ],
            "tests/datasets/mapreduce/merge_first_second_doc.pdf",
            Direction::Vertical
        )
    );
}
