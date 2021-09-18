use std::fs::File;
use std::path::PathBuf;
use std::io;
use std::process::exit;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Cli {
    img_file: PathBuf,
    #[structopt(parse(from_os_str))]
    other_img_files: Vec<PathBuf>
}

fn main() {
    let args = Cli::from_args();

    let img_files = match open_img_files(&args.img_file, args.other_img_files) {
        Ok(files) => files,
        Err(_) => exit(-1),
    };

    println!("{:?}", img_files);
}

// Open VM image files
fn open_img_files(img_file: &PathBuf, other_img_files: Vec<PathBuf>) -> Result<Vec<File>, io::Error> {

    let mut img_files: Vec<File> = Vec::with_capacity(1 + other_img_files.len());

    match File::open(&img_file) {
        Err(error) => {
                eprintln!("error: couldn't open {:?}, {}", img_file, error);
                return Err(error)
            },
        Ok(file) => img_files.push(file)
    }

    for img_file_path in other_img_files.iter() {
        match File::open(&img_file_path) {
            Err(error) => {
                eprintln!("error: couldn't open {:?}, {}", img_file_path, error);
                return Err(error)
            },
            Ok(file) => img_files.push(file)
        }
    }

    return Ok(img_files)
}
