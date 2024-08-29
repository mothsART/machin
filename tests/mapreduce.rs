use std::env;
use std::fs::File;
use std::io::Read;

use map_macro::hash_map;
use sha1::{Digest, Sha1};

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
        "386da50e983fc714b1e2211e978c9d3ea8da8d5e",
        get_hash_after(
            vec![
                "tests/datasets/rusted_chain__with_alpha.jpg",
                "tests/datasets/rusted_gears.jpg",
            ],
            "tests/datasets/mapreduce/rusted_chain__with_alpha__vertical__rusted_gears.jpg",
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
                "tests/datasets/rusted_chain__with_alpha.jpg",
                "tests/datasets/rusted_gears.jpg",
            ],
            "tests/datasets/mapreduce/rusted_chain__with_alpha__horizontal__rusted_gears.jpg",
            Direction::Horizontal
        )
    );
}

#[test]
fn create_png_with_heterogeneous_pictures() {
    assert_eq!(
        "7864e925834b66598d4cbbce719d6b55e35828a4",
        get_hash_after(
            vec![
                "tests/datasets/rusted_gears.jpg",
                "tests/datasets/car-vintage-old-rusty.png",
            ],
            "tests/datasets/mapreduce/with_heterogeneous_pictures.png",
            Direction::Vertical
        )
    );
}

#[test]
fn create_pdf_with_heterogeneous_pictures() {
    assert_eq!(
        "b2a7e9ba4f23291f21fbd0b382720e72ef32258e",
        get_hash_after(
            vec![
                "tests/datasets/rusted_gears.jpg",
                "tests/datasets/car-vintage-old-rusty.png",
            ],
            "tests/datasets/mapreduce/with_heterogeneous_pictures.pdf",
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
