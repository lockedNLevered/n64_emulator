const NUM_GPR: usize = 32;

#[derive(Default, Debug)]
pub struct Cpu {

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
    pub fn hard_reset(&mut self) {
            self.cp0.hard_reset(); 
    }
}

#[derive(Debug)]
enum RegConfigEp {
    D, //TODO: Change name?
    DxxDxx, 
    RFU
}
impl Default for RegConfigEp {
    fn default() -> RegConfigEp {
        RegConfigEp::D
    }
}

#[derive(Debug)]
enum RegConfigBe {
    LittleEndian,
    BigEndian,
}
impl Default for RegConfigBe {
    fn default() -> RegConfigBe {
        RegConfigBe::BigEndian
    }
}

#[derive(Default, Debug)]
struct RegConfig {
    reg_config_ep: RegConfigEp,
    reg_config_be: RegConfigBe
}

impl RegConfig {
    fn hard_reset(&mut self) {
        self.reg_config_ep = RegConfigEp::D;
        self.reg_config_be = RegConfigBe::BigEndian;

    }
}

#[derive(Default, Debug)]
struct Cp0 {
    reg_config: RegConfig

}
impl Cp0 {
    
    fn hard_reset(&mut self) {
        self.reg_config.hard_reset();
    }
}