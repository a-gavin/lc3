/* ~~~ Constants and enums ~~~ */
const UINT16_MAX: usize = 65536;
const PRGM_START_ADDR: usize = 0x3000;

#[derive(Debug, Enum)]
enum GpRegister {
    R0, R1, R2, R3, R4, R5, R6, R7
}

#[derive(Debug, Enum)]
enum Flag {
    POS,
    ZRO,
    NEG
}

#[derive(Debug)]
enum Instr {
    BR,     // Branch
    ADD,    // Add
    LD,     // Load
    ST,     // Store
    JSR,    // Jump register
    AND,    // Bitwise AND
    LDR,    // Load register
    STR,    // Store register
    RTI,    // Unused
    NOT,    // Bitwise NOT
    LDI,    // Load indirect
    STI,    // Store indirect
    JMP,    // Jump
    RES,    // Reserved (unused)
    LEA,    // Load effective address
    TRAP    // Execute trap
}


/* ~~~ Imports ~~~ */
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process::exit;

use structopt::StructOpt;
use enum_map::{enum_map, Enum, EnumMap};

use Flag::*;
use GpRegister::*;
use Instr::*;


/* ~~~ Structs ~~~ */
// CLI Parsing
#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Cli {
    #[structopt(parse(from_os_str))]
    program_file: PathBuf,
}

// LC-3 VM
pub struct LC3 {
    memory: Option<[u8; UINT16_MAX]>,
    gp_regs: EnumMap<GpRegister, u16>,
    pc: usize,
    cond: Flag
}

impl Default for LC3 {
    fn default() -> LC3 {
        LC3 {
            memory: None,
            gp_regs: enum_map! {
                R0 => 0,
                R1 => 0,
                R2 => 0,
                R3 => 0,
                R4 => 0,
                R5 => 0,
                R6 => 0,
                R7 => 0,
            },
            pc: PRGM_START_ADDR,
            cond: ZRO
        }
    }
}

impl LC3 {
    pub fn new(program_file: PathBuf) -> LC3 {
        // Open VM image file
        let mut program_file = match File::open(&program_file) {
            Ok(file) => file,
            Err(error) => {
                eprintln!("error: couldn't open {:?}, {}", program_file, error);
                exit(-1)
            }
        };

        // Read program into memory
        let mut memory: [u8; UINT16_MAX] = [0; UINT16_MAX];
        let n = match program_file.read(&mut memory[PRGM_START_ADDR..]) {
            Ok(n) => n,
            Err(error) => {
                eprintln!("error: couldn't read {:?}, {}", program_file, error);
                exit(-1)
            }
        };
        assert!(n < (UINT16_MAX - PRGM_START_ADDR), "Cannot run program of size larger than {}", UINT16_MAX - PRGM_START_ADDR);


        LC3 { memory: Some(memory), ..Default::default() }
    }

    pub fn run(self) {
        println!("memory: {:?}", self.memory.unwrap());
        println!("gp_regs: {:?}", self.gp_regs);
        println!("PC: {:#04x}", self.pc);
        println!("COND: {:?}", self.cond);

        // Get op
        let op: Instr = ADD;

        loop {
            match op {
                Instr::ADD  => println!(),
                Instr::AND  => println!(),
                Instr::NOT  => println!(),
                Instr::BR   => println!(),
                Instr::JMP  => println!(),
                Instr::JSR  => println!(),
                Instr::LD   => println!(),
                Instr::LDI  => println!(),
                Instr::LDR  => println!(),
                Instr::LEA  => println!(),
                Instr::ST   => println!(),
                Instr::STI  => println!(),
                Instr::STR  => println!(),
                Instr::TRAP => println!(),
                Instr::RES  => println!(),
                Instr::RTI  => println!(),
            }

            break;
        }
    }
}


/* ~~~ The fun stuff ~~~ */
fn main() {
    let args = Cli::from_args();
    let lc3 = LC3::new(args.program_file);
    lc3.run();
}