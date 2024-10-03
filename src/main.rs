#[path = "instructions/cbprefixed.rs"]
mod cbprefixed;
mod cpu;
mod helper_funcs;
mod memory;
mod registers;
#[path = "instructions/unprefixed.rs"]
mod unprefixed;

use cpu::CPU;

use std::env;
use std::path::PathBuf;

const PATH_TO_ROM: &str = "";

fn main() {
    let mut CPU = CPU::new();

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let rom_path = PathBuf::from(manifest_dir).join(PATH_TO_ROM);

    CPU.load_instructions(rom_path.to_str().unwrap());
}
