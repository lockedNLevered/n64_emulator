use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

const NUM_GPR: usize = 32;

struct Cpu {

    gpr_registers: [u64; NUM_GPR],
    fpr_registers: [f64; NUM_GPR],

   
    pc_register: u64,

    hi_register: u64,
    lo_register: u64,

    load_link_register: bool, //TODO

    fcr0_register: u32,
    fcr31_register: u32,
}

impl Cpu { 
    fn new() -> Cpu {
        Cpu {
            //Fil this
        }
    }

}

fn main() {
    
    let pif_file_name = env::args().nth(1).unwrap();
    let rom_file_name = env::args().nth(2).unwrap();
    
    let pif = read_binary(pif_file_name);
    let rom =read_binary(rom_file_name);
    
    let mut cpu = Cpu::new();
}

fn read_binary<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = File::open(path.as_ref()).unwrap();
    let mut file_buffer = Vec::new();
    file.read_to_end(&mut file_buffer).unwrap();
    file_buffer 
}