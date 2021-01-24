mod cpu;

use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;



fn main() {
    let pif_file_name = env::args().nth(0).unwrap();
    let rom_file_name = env::args().nth(1).unwrap();
    
    let pif = read_bin(pif_file_name);
    let rom = read_bin(rom_file_name);
    
    let mut cpu = cpu::Cpu::default();
    cpu.hard_reset(); 
    println!("{:#?}", &cpu);
}

fn read_bin<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = fs::File::open(path).unwrap();
    let mut file_buffer = Vec::new();
    file.read_to_end(&mut file_buffer).unwrap();
    file_buffer 
}