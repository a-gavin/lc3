use std::fs::File;
use std::path::PathBuf;
use std::process::exit;
use structopt::StructOpt;


// CLI Parsing
#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Cli {
    #[structopt(parse(from_os_str))]
    program_file: PathBuf,
}

// LC-3 VM
pub struct LC3 {
    program_file: File
}

impl LC3 {
    pub fn new(program_file: PathBuf) -> LC3 {
        // Open VM image file
        let program_file = match File::open(&program_file) {
            Ok(file) => file,
            Err(error) => {
                eprintln!("error: couldn't open {:?}, {}", program_file, error);
                exit(-1)
            }
        };

        LC3 {
            program_file: program_file
        }
    }
}


fn main() {
    let args = Cli::from_args();
    let lc3 = LC3::new(args.program_file);

    println!("{:?}", lc3.program_file);
}