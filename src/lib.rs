/* ~~~ LC-3 VM ~~~ */
pub mod lc3 {
    use std::process::exit;
    use std::path::PathBuf;

    use util::*;
    use op_code::*;

    /* Constants and enums */
    const NUM_GP_REGS: usize = 8;
    const UINT16_MAX: usize = 65536;
    const PRGM_START_ADDR: usize = 0x3000;

    // Cond bits: XXXXXPZN, where X indicates unused
    pub mod flag {
        pub const POS: u8 = 0b0100;
        pub const ZRO: u8 = 0b0010;
        pub const NEG: u8 = 0b0001;
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

    /* LC3 implementation */
    pub struct LC3 {
        memory: [u16; UINT16_MAX],
        gp_regs: [u16; NUM_GP_REGS],
        pc: usize,
        cond: u8,
        debug: bool
    }

    impl Default for LC3 {
        fn default() -> LC3 {
            LC3 {
                memory: [0; UINT16_MAX],
                gp_regs: [0; NUM_GP_REGS],
                pc: PRGM_START_ADDR,
                cond: 0,
                debug: false
            }
        }
    }

    impl LC3 {
        pub fn new(program_file: &PathBuf, debug: bool) -> LC3 {
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
                    debug: debug,
                    ..Default::default()
                }
        }

        pub fn run(mut self) {
            if self.debug {
                println!("memory:");
                for i in (0..10).step_by(1) {
                    println!("{:#04x}: {:08b} {:08b}",
                            self.pc + i,
                            (self.memory[self.pc+i] >> 8) & 0x00FF,
                            self.memory[self.pc+i] & 0x00FF);
                }
                println!();
            }

            loop {
                if !self.fetch_and_exec() {
                    break;
                }
            }
        }

        fn fetch_and_exec(&mut self) -> bool {
            // Fetch instruction and advance PC
            let instr = self.mem_read(self.pc);
            let opcode = (instr >> 12) as u8;

            if self.debug {
                println!("PC:      {:#04x}\n\
                          OPCODE:  {:#06b}\n\
                          COND:    {:#010b}\n\
                          gp_regs: {:?}\n",
                          self.pc,
                          opcode,
                          self.cond,
                          self.gp_regs);
            }
            self.pc += 1;

            // Perform fetched operation
            match opcode {
                ADD  => self.add(instr),
                AND  => self.and(instr),
                BR   => println!(),
                JMP  => self.jmp(instr),
                JSR  => self.jsr(instr),
                LDB  => println!(),
                LDI  => self.ldi(instr),
                LDR  => println!(),
                LEA  => self.lea(instr),
                NOT  => self.not(instr),
                STB  => println!(),
                STI  => println!(),
                STR  => println!(),
                TRAP => return false,
                SHF  => println!(),
                _ => return false, // All others, including Opcode::RTI
            }

            return true;
        }

        fn add(&mut self, instr: u16) {
            let dest_reg = get_reg(instr, 9);
            let op1_reg = get_reg(instr, 5);

            if get_immed_bit(instr) == 1 {
                // Immediate ADD
                let imm5 = sign_extend(instr & 0x1F, 5);
                self.gp_regs[dest_reg as usize] = self.gp_regs[op1_reg as usize] + imm5;
            }
            else {
                // Register ADD
                let op2_reg: u16 = get_reg(instr, 0);
                self.gp_regs[dest_reg as usize] = self.gp_regs[op1_reg as usize] + self.gp_regs[op2_reg as usize];
            }

            update_flags(dest_reg, &mut self.cond);
        }

        fn and(&mut self, instr: u16) {
            let dest_reg = get_reg(instr, 9);
            let op1_reg = get_reg(instr, 6);

            if get_immed_bit(instr) == 1 {
                // Immediate AND
                let imm5 = sign_extend(instr & 0x1F, 5);
                self.gp_regs[dest_reg as usize] = self.gp_regs[op1_reg as usize] & imm5;
            }
            else {
                // Register AND
                let op2_reg: u16 = get_reg(instr, 0);
                self.gp_regs[dest_reg as usize] = self.gp_regs[op1_reg as usize] & self.gp_regs[op2_reg as usize];
            }

            update_flags(dest_reg, &mut self.cond);
        }

        // TODO: Verify
        fn jmp(&mut self, instr: u16) {
            let offset_reg = get_reg(instr, 6) as usize;
            self.pc = self.gp_regs[offset_reg] as usize;
        }

        // TODO: Verify
        fn jsr(&mut self, instr: u16) {
            let long_flag = (instr >> 1) & 1;

            // Save jump address to register R7
            self.gp_regs[7] = self.pc as u16;

            if long_flag == 1 {
                // JSR
                let long_pc_offset = sign_extend(instr & 0x7FF, 11);
                self.pc += long_pc_offset as usize;
            }
            else {
                // JSRR
                // TODO: Verify
                let op1_reg = get_reg(instr, 6);
                self.pc = self.gp_regs[op1_reg as usize] as usize;
            }
        }

        // TODO: Verify
        fn lea(&mut self, instr: u16) {
            let dest_reg = get_reg(instr, 9);
            let pc_offset = sign_extend(instr & 0x1FF, 9);

            self.gp_regs[dest_reg as usize] = self.pc as u16 + pc_offset;

            update_flags(dest_reg, &mut self.cond);
        }

        // TODO: Verify
        fn ldi(&mut self, instr: u16) {
            let dest_reg = get_reg(instr, 9);
            let pc_offset = sign_extend(instr & 0x1FF, 9);

            let addr = self.pc as usize + pc_offset as usize;

            self.gp_regs[dest_reg as usize] = self.mem_read(self.mem_read(addr) as usize);

            update_flags(dest_reg, &mut self.cond);
        }

        fn not(&mut self, instr: u16) {
            let dest_reg = get_reg(instr, 9);
            let op1_reg = get_reg(instr, 6);

            self.gp_regs[dest_reg as usize] = !self.gp_regs[op1_reg as usize];

            update_flags(dest_reg, &mut self.cond);
        }

        // Read halfword of memory (16 bits) at location "reg_val"
        fn mem_read(&self, reg_val: usize) -> u16 {
            self.memory[reg_val]
        }
    }

    /* Module utility functions */
    mod util {
        use std::fs::File;
        use std::io;
        use std::io::Read;
        use std::path::PathBuf;

        use crate::lc3::flag;
        use crate::lc3::{UINT16_MAX, PRGM_START_ADDR};


        pub fn is_negative(value: u16) -> bool {
            (value >> 15) == 1
        }

        pub fn get_as_u16(upper_byte: u8, lower_byte: u8) -> u16 {
            ((upper_byte as u16) << 8) | lower_byte as u16
        }

        pub fn get_immed_bit(instr: u16) -> u16 {
            (instr >> 5) & 0x1
        }

        pub fn get_reg(instr: u16, shift: u16) -> u16 {
            (instr >> shift) & 0x7
        }

        pub fn read_program_file(program_file: &PathBuf) -> Result<(usize, [u16; UINT16_MAX]), io::Error> {
            // Open VM image file
            let mut program_file = File::open(&program_file)?;
            let file_handle = program_file.by_ref();

            // Read program start location
            let mut two_byte_chunk = Vec::with_capacity(2);
            file_handle.take(2).read_to_end(&mut two_byte_chunk)?;

            // Convert to little endian; if zero, use default
            let read_start_addr = get_as_u16(two_byte_chunk[0], two_byte_chunk[1]);

            let mut start_addr = PRGM_START_ADDR;
            if read_start_addr != 0 {
                start_addr = read_start_addr as usize;
            }

            // Read program into memory, starting at program start location
            //  Assumes in correct big-endian byte order.
            //  Code written for easy swap to little-endian, if necessary
            let mut memory: [u16; UINT16_MAX] = [0; UINT16_MAX];
            let mut ix = start_addr;
            two_byte_chunk.clear();

            loop {
                let n = program_file.by_ref()
                                    .take(2 as u64)
                                    .read_to_end(&mut two_byte_chunk)?;
                if n == 0 { break; }

                // Copy into memory
                memory[ix] = get_as_u16(two_byte_chunk[0], two_byte_chunk[1]);
                two_byte_chunk.clear();
                ix += 1;
            }

            Ok((start_addr, memory))
        }

        pub fn sign_extend(register: u16, num_bits: u8) -> u16 {
            if ((register >> (num_bits - 1)) & 1) == 1{
                return register | (0xFFFF << num_bits);
            }
            register
        }

        pub fn update_flags(register_val: u16, cond: &mut u8) {
            if register_val == 0 {
                *cond &= flag::ZRO;
            }
            else if is_negative(register_val) {
                *cond &= flag::NEG;
            }
            else {
                *cond &= flag::POS;
            }
        }
    }
}