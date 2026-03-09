use sp1_build::BuildArgs;
use std::{
    fs,
    path::Path,
};

const DOCKER_BUILD_ENV: &str = "SP1_REPRO_DOCKER_BUILD";
const WORKSPACE_DIRECTORY: &str = "..";
const ELF_BUILD_DIRECTORY: &str = "target/elf-compilation/riscv64im-succinct-zkvm-elf/release";

fn build_args(docker: bool) -> BuildArgs {
    BuildArgs {
        docker,
        workspace_directory: Some(WORKSPACE_DIRECTORY.to_string()),
        ..Default::default()
    }
}

fn write_placeholder_elf(program_dir: &str, binary_name: &str) {
    let elf_path = Path::new(program_dir)
        .join(ELF_BUILD_DIRECTORY)
        .join(binary_name);
    let elf_dir = elf_path
        .parent()
        .expect("ELF path should have a parent directory");

    fs::create_dir_all(elf_dir).expect("failed to create placeholder ELF directory");
    fs::write(&elf_path, []).expect("failed to create placeholder ELF");
}

fn main() {
    println!("cargo::rerun-if-env-changed={DOCKER_BUILD_ENV}");
    println!("cargo::rerun-if-changed=../program/fibonacci");
    println!("cargo::rerun-if-changed=../program/aggregator");

    let docker = std::env::var(DOCKER_BUILD_ENV).is_ok();
    sp1_build::build_program_with_args("../program/fibonacci", build_args(docker));
    sp1_build::build_program_with_args("../program/aggregator", build_args(docker));

    let is_clippy_driver = std::env::var("RUSTC_WORKSPACE_WRAPPER")
        .map(|value| value.contains("clippy-driver"))
        .unwrap_or(false);
    if is_clippy_driver {
        write_placeholder_elf("../program/fibonacci", "repro_fibonacci_program");
        write_placeholder_elf("../program/aggregator", "repro_aggregator_program");
    }
}
