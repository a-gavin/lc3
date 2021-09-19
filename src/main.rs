use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process::exit;
use structopt::StructOpt;

// Constants
const UINT16_MAX: usize = 65536;
const PRGM_START_ADDR: usize = 12288;


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
}

impl Default for LC3 {
    fn default() -> LC3 {
        LC3 {
            memory: None,
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


        LC3 {
            memory: Some(memory)
        }
    }
}


fn main() {
    let args = Cli::from_args();
    let lc3 = LC3::new(args.program_file);

    println!("memory: {:?}", lc3.memory.unwrap());
}