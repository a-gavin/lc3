/* ~~~ Imports ~~~ */
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;
use std::process::exit;

use structopt::StructOpt;

use op_code::*;


/* ~~~ Constants and enums ~~~ */
const NUM_GP_REGS: usize = 8;
const UINT16_MAX: usize = 65536;
const PRGM_START_ADDR: usize = 0x3000;

#[derive(Debug)]
enum Flag {
    POS,
    ZRO,
    NEG
}

pub mod op_code {
    pub const BR: u8 = 0b000;       // Branch
    pub const ADD: u8 = 0b0001;     // Add
    pub const LDB: u8 = 0b0010;     // Load
    pub const STB: u8 = 0b0011;     // Store
    pub const JSR: u8 = 0b0100;     // Jump register
    pub const AND: u8 = 0b0101;     // Bitwise AND
    pub const LDR: u8 = 0b0110;     // Load register
    pub const STR: u8 = 0b0111;     // Store register
    pub const RTI: u8 = 0b1000;     // Unused
    pub const NOT: u8 = 0b1001;     // Bitwise NOT
    pub const LDI: u8 = 0b1010;     // Load indirect
    pub const STI: u8 = 0b1011;     // Store indirect
    pub const JMP: u8 = 0b1100;     // Jump
    pub const SHF: u8 = 0b1101;     // Shift register
    pub const LEA: u8 = 0b1110;     // Load effective address   
    pub const TRAP: u8 = 0b1111;    // Execute trap
}


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
    gp_regs: [u16; NUM_GP_REGS],
    pc: usize,
    cond: Flag
}

impl Default for LC3 {
    fn default() -> LC3 {
        LC3 {
            memory: [0; UINT16_MAX],
            gp_regs: [0; NUM_GP_REGS],
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
            let opcode = (instr >> 12) as u8;

            println!("PC: {:#04x}", self.pc);
            println!("OPCODE: {:?}", opcode);
            println!("COND: {:?}", self.cond);
            println!("gp_regs: {:?}", self.gp_regs);

            self.pc += 1;
            match opcode {
                ADD  => println!(),
                AND  => println!(),
                BR   => println!(),
                JMP  => println!(),
                JSR  => println!(),
                LDB  => println!(),
                LDI  => println!(),
                LDR  => println!(),
                LEA  => println!(),
                NOT  => println!(),
                STB  => println!(),
                STI  => println!(),
                STR  => println!(),
                TRAP => println!(),
                SHF  => println!(),
                _ => return, // All others, including Opcode::RTI
            }
        }
    }

    fn mem_read(&self, reg_val: usize) -> u16 {
        swap_bytes(self.memory[reg_val], self.memory[reg_val+1])
    }
}


/* ~~~ The fun stuff ~~~ */
fn main() {
    let args = Cli::from_args();
    let lc3 = LC3::new(&args.program_file);
    lc3.run();
}

/* Utility functions */
fn is_negative(value: u16) -> bool {
    (value >> 15) == 1
}

fn get_immed_bit(instr: u16) -> u16 {
    (instr >> 5) & 0x1
}

fn read_program_file(program_file: &PathBuf) -> Result<(usize, [u8; UINT16_MAX]), io::Error> {
    // Open VM image file
    let mut program_file = File::open(&program_file)?;
    let file_handle = program_file.by_ref();

    //// Read program start location
    let mut two_byte_chunk = Vec::with_capacity(2);
    file_handle.take(2).read_to_end(&mut two_byte_chunk)?;

    // Convert to little endian; if zero, use default
    let read_start_addr = swap_bytes(two_byte_chunk[0], two_byte_chunk[1]);

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

fn sign_extend(register: u16, num_bits: u8) -> u16 {
    if ((register >> (num_bits - 1)) & 1) == 1{
        return register | (0xFFFF << num_bits);
    }
    register
}

fn swap_bytes(op1: u8, op2: u8) -> u16 {
    ((op1 as u16) << 8) | op2 as u16
}

fn update_flags(register_val: u16, cond: &mut Flag) {
    if register_val == 0 {
        *cond = Flag::ZRO;
    }
    else if is_negative(register_val) {
        *cond = Flag::NEG;
    }
    else {
        *cond = Flag::POS;
    }
}
