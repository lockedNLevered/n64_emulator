use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

const NUM_GPR: usize = 32;

#[derive(Default)]
struct Cpu {

    register_gpr: [u64; NUM_GPR],
    register_fpr: [f64; NUM_GPR],

   
    register_pc: u64,

    register_hi: u64,
    register_lo: u64,

    register_load_link: bool, //TODO add enum type

    register_fcr0: u32,
    register_fcr31: u32,

    cp0: Cp0
}

impl Cpu { 
        fn new() -> Cpu {
            Cpu::default()
        }

        fn hard_reset(&mut self) {
                self.cp0.hard_reset(); 
        }
    }

#[derive(Default)]
struct Cp0 {
    reg_index: u64,
    reg_random: u64,
    reg_entrylo0: u64,
    reg_entrylo1: u64,
    reg_context: u64,
    reg_page_mask: u64,
    reg_wired: u64,
    reg_bad_v_addr: u64,
    reg_count: u64,
    reg_entry_hi: u64,
    reg_compare: u64,
    reg_status: u64,
    reg_cause: u64,
    reg_epc: u64,
    reg_prid: u64,
    reg_config: u64,
    reg_load_link_address: u64,
    reg_watch_lo: u64,
    reg_watch_hi: u64,
    reg_x_context: u64,
    reg_tag_lo: u64,
    reg_tag_hi: u64,
    reg_error_epc: u64
}
impl Cp0 {
    fn new() -> Cp0 {
        Cp0::default()
    }
    fn hard_reset(&mut self) {
        //todo
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