use std::path::PathBuf;
use structopt::StructOpt;

use lc3::lc3::LC3;


// CLI Parsing
#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Cli {
    #[structopt(short, long)]
    debug: bool,
    #[structopt(parse(from_os_str))]
    program_file: PathBuf,
}


/* ~~~ The fun stuff ~~~ */
fn main() {
    let args = Cli::from_args();
    let vm = LC3::new(&args.program_file, args.debug);
    vm.run();
}