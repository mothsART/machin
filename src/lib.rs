pub mod errors;
#[macro_use]
pub mod macros;
pub mod machconvert;
pub mod machmap;
pub mod machreduce;

pub fn readlines() -> Vec<String> {
    let stdin = std::io::stdin();
    let v = stdin.lines().map(|x| x.unwrap()).collect();
    v
}
