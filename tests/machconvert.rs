use std::env;

use sha1::{Digest, Sha1};
use tempfile::tempdir;

use machin::machconvert::{ConvertArgs, ConvertColor, ConvertFlip, InputsFiles};

fn get_hash_after(
    input_path: &'static str,
    output_file: &'static str,
    args: &ConvertArgs,
) -> String {
    let path = env::current_dir().unwrap();
    let input_file = path.join(input_path).display().to_string();

    let tmp_dir = tempdir().unwrap();
    let output_path = tmp_dir.path().join(output_file).display().to_string();

    InputsFiles::new(&input_file, &output_path)
        .convert(args)
        .unwrap();

    let bytes = std::fs::read(&output_path).unwrap();
    let mut hasher = Sha1::new();
    hasher.update(&bytes);

    let str_hash = format!("{:x}", hasher.finalize());
    tmp_dir.close().unwrap();

    return str_hash;
}

///
/// In order :
/// - grayscale
/// - vertical flip
/// - 90 degree rotation
///
#[test]
fn png_grayscale_vertical_rotate() {
    let args = ConvertArgs {
        color: Some(ConvertColor::Grayscale),
        flip: Some(ConvertFlip::Vertical),
        rotate: Some(90),
    };
    assert_eq!(
        "e16f1ffa1a40302c8d3cd486f9bcdc03cfc6892b",
        get_hash_after(
            "tests/datasets/car-vintage-old-rusty.png",
            "car-vintage-old-rusty.png",
            &args
        )
    );
}

///
/// In order :
/// - vertical horizontal
/// - 270 degree rotation
///
#[test]
fn jpg_grayscale_vertical_rotate() {
    let args = ConvertArgs {
        color: None,
        flip: Some(ConvertFlip::Horizontal),
        rotate: Some(270),
    };
    assert_eq!(
        "5f8d1cce382e45cff88247ef2914dd728128d013",
        get_hash_after("tests/datasets/rusted_chain.jpg", "rusted_chain.jpg", &args)
    );
}
