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
enum Opcode {
    BR,     // Branch
    ADD,    // Add
    LDB,    // Load
    STB,    // Store
    JSR,    // Jump register
    AND,    // Bitwise AND
    LDR,    // Load register
    STR,    // Store register
    RTI,    // Unused
    NOT,    // Bitwise NOT
    LDI,    // Load indirect
    STI,    // Store indirect
    JMP,    // Jump
    SHF,    // Bit shift
    LEA,    // Load effective address
    TRAP,   // Execute trap
    ERR     // For invalid opcodes
}


/* ~~~ Imports ~~~ */
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;
use std::process::exit;

use structopt::StructOpt;
use enum_map::{enum_map, Enum, EnumMap};


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
    memory: [u8; UINT16_MAX],
    gp_regs: EnumMap<GpRegister, u16>,
    pc: usize,
    cond: Flag
}

impl Default for LC3 {
    fn default() -> LC3 {
        LC3 {
            memory: [0; UINT16_MAX],
            gp_regs: enum_map! {
                GpRegister::R0 => 0,
                GpRegister::R1 => 0,
                GpRegister::R2 => 0,
                GpRegister::R3 => 0,
                GpRegister::R4 => 0,
                GpRegister::R5 => 0,
                GpRegister::R6 => 0,
                GpRegister::R7 => 0,
            },
            pc: PRGM_START_ADDR,
            cond: Flag::ZRO
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
                memory: memory,
                ..Default::default()
            }
    }

    pub fn run(mut self) {
        println!("memory: {:?}", &self.memory[(self.pc-10)..(self.pc+100)]);

        loop {
            // Fetch instruction and advance PC
            let instr = self.mem_read(self.pc);
            let opcode = Self::get_unpack_opcode(instr);

            println!("PC: {:#04x}", self.pc);
            println!("OPCODE: {:?}", opcode);
            println!("COND: {:?}", self.cond);
            println!("gp_regs: {:?}", self.gp_regs);

            self.pc += 1;
            match opcode {
                Opcode::ADD  => println!(),
                Opcode::AND  => println!(),
                Opcode::BR   => println!(),
                Opcode::JMP  => println!(),
                Opcode::JSR  => println!(),
                Opcode::LDB  => println!(),
                Opcode::LDI  => println!(),
                Opcode::LDR  => println!(),
                Opcode::LEA  => println!(),
                Opcode::NOT  => println!(),
                Opcode::STB  => println!(),
                Opcode::STI  => println!(),
                Opcode::STR  => println!(),
                Opcode::TRAP => println!(),
                Opcode::SHF  => println!(),
                _ => return, // All others, including Opcode::RTI
            }
        }
    }

    fn mem_read(&self, reg_val: usize) -> u16 {
        ((self.memory[reg_val] as u16) << 8) | self.memory[reg_val+1] as u16
    }

    fn get_unpack_opcode(instr: u16) -> Opcode {
        let opcode_int = (instr >> 12) as u8;

        match opcode_int {
            0b0000 => Opcode::BR,
            0b0001 => Opcode::ADD,
            0b0010 => Opcode::LDB,
            0b0011 => Opcode::STB,
            0b0100 => Opcode::JSR,
            0b0101 => Opcode::AND,
            0b0110 => Opcode::LDR,
            0b0111 => Opcode::STR,
            0b1000 => Opcode::RTI,
            0b1001 => Opcode::NOT,
            0b1010 => Opcode::LDI,
            0b1011 => Opcode::STI,
            0b1100 => Opcode::JMP,
            0b1101 => Opcode::SHF,
            0b1110 => Opcode::LEA,
            0b1111 => Opcode::TRAP,
            _      => Opcode::ERR
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