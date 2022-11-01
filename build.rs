#[macro_use]
extern crate clap;

use std::env;

use clap::{Arg, Command};
use clap_complete::{generate_to, shells::Bash, shells::Fish, shells::Zsh};

include!("src/machconvert/cli.rs");
include!("src/machmap/cli.rs");
include!("src/machreduce/cli.rs");

fn main() -> Result<(), std::io::Error> {
    let outdir = env!("CARGO_MANIFEST_DIR");

    let machconvert_name = "machconvert";
    let mut machconvert_cmd =
        build_machconvert_cli(machconvert_name, crate_version!(), crate_authors!());
    generate_to(Bash, &mut machconvert_cmd, machconvert_name, &outdir)?;
    generate_to(Zsh, &mut machconvert_cmd, machconvert_name, &outdir)?;
    generate_to(Fish, &mut machconvert_cmd, machconvert_name, &outdir)?;

    let machmap_name = "machmap";
    let mut machmap_cmd = build_machmap_cli(machmap_name, crate_version!(), crate_authors!());
    generate_to(Bash, &mut machmap_cmd, machmap_name, &outdir)?;
    generate_to(Zsh, &mut machmap_cmd, machmap_name, &outdir)?;
    generate_to(Fish, &mut machmap_cmd, machmap_name, &outdir)?;

    let machreduce_name = "machreduce";
    let mut machreduce_cmd =
        build_machreduce_cli(machreduce_name, crate_version!(), crate_authors!());
    generate_to(Bash, &mut machreduce_cmd, machreduce_name, &outdir)?;
    generate_to(Zsh, &mut machreduce_cmd, machreduce_name, &outdir)?;
    generate_to(Fish, &mut machreduce_cmd, machreduce_name, &outdir)?;

    Ok(())
}
