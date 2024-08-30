use std::env;

use sha1::{Digest, Sha1};

use machin::machconvert::{ConvertArgs, ConvertColor, ConvertFlip, InputsFiles};

///
/// get hash of image after operation
///
fn get_hash_after(
    input_path: &'static str,
    output_file: &'static str,
    args: &ConvertArgs,
) -> String {
    let path = env::current_dir().unwrap();
    let input_file = path.join(input_path).display().to_string();
    let output_path = path.join(output_file).display().to_string();
    InputsFiles::new(&input_file, &output_path)
        .convert(args)
        .unwrap();

    let bytes = std::fs::read(&output_path).unwrap();
    let mut hasher = Sha1::new();
    hasher.update(&bytes);

    let str_hash = format!("{:x}", hasher.finalize());
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
        "9ba1ce2b3a95ee59b6979a24e9b62c4c0a221c87",
        get_hash_after(
            "tests/datasets/car-vintage-old-rusty__with_alpha.png",
            "tests/datasets/machconvert/car-vintage-old-rusty__with_alpha.png",
            &args,
        )
    );
    assert_eq!(
        "651978d8b41fb628eed0ffd2bb58d39e40ee2fbd",
        get_hash_after(
            "tests/datasets/car-vintage-old-rusty__without_alpha.png",
            "tests/datasets/machconvert/car-vintage-old-rusty__without_alpha.png",
            &args,
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
        "908702b6429c033b142f0d23b746d85436d6f242",
        get_hash_after(
            "tests/datasets/rusted_chain.jpg",
            "tests/datasets/machconvert/rusted_chain__grayscale_vertical_rotate.jpg",
            &args,
        )
    );
}
