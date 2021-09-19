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
use std::io;
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
    pub fn new(program_file: &PathBuf) -> LC3 {
        let (start_addr, memory) = match read_program_file(program_file) {
            Ok(n) => n,
            Err(error) => {
                eprintln!("error: couldn't read {:?}, {}", program_file, error);
                exit(-1);
            }
        };

        LC3 {
                pc: start_addr,
                memory: Some(memory),
                ..Default::default()
            }
    }

    pub fn run(self) {
        println!("memory: {:?}", &self.memory.unwrap()[(self.pc-10)..(self.pc+100)]);
        println!("gp_regs: {:?}", self.gp_regs);
        println!("PC: {:#04x}", self.pc);
        println!("COND: {:?}", self.cond);

        // Get op
        let op: Instr = ADD;
        println!("{:04x}", self.memory.unwrap()[PRGM_START_ADDR]);

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
    let lc3 = LC3::new(&args.program_file);
    lc3.run();
}

fn read_program_file(program_file: &PathBuf) -> Result<(usize, [u8; UINT16_MAX]), io::Error> {
    // Open VM image file
    let mut program_file = File::open(&program_file)?;
    let file_handle = program_file.by_ref();

    //// Read program start location
    let mut two_byte_chunk = Vec::with_capacity(2);
    file_handle.take(2).read_to_end(&mut two_byte_chunk)?;

    // Convert to little endian; if zero, use default
    let read_start_addr = ((two_byte_chunk[0] as u16) << 8) | two_byte_chunk[1] as u16;

    let mut start_addr = PRGM_START_ADDR;
    if read_start_addr != 0 {
        start_addr = read_start_addr as usize;
    }

    // Read program into memory, starting at program start location, swapping to little endian
    let mut memory: [u8; UINT16_MAX] = [0; UINT16_MAX];
    let mut ix = start_addr;
    two_byte_chunk.clear();

    loop {
        let n = program_file.by_ref()
                            .take(2 as u64)
                            .read_to_end(&mut two_byte_chunk)?;
        if n == 0 { break; }

        // Swap and copy into memory
        memory[ix] = two_byte_chunk[1];
        memory[ix+1] = two_byte_chunk[0];
        two_byte_chunk.clear();

        ix += 2;
    }

    Ok((start_addr, memory))
}