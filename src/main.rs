use clap::Parser;
use std::fs;
use std::path::PathBuf;

// A program that outputs True if the given file has a sha256 struct equal to the sha56 digest passed into the command
#[derive(Parser, Default)]
#[command(version, about)]
struct Args {
    // The fie path which is being checked
    #[arg(required = true)]
    file_path: PathBuf,

    // Providing an md5 sum
    #[arg(long, required = false)]
    md5: Option<String>,

    // Providing a sha256 sum
    #[arg(long, required = false)]
    sha256: Option<String>,
}

fn main() {
    let args = Args::parse();

    let file_data = fs::read(&args.file_path)
        .expect("Error: The provided file could not be converted into a bytes vector.");

    match args.md5 {
        Some(md5_given) => {
            let md5_sum = format!("{:x}", md5::compute(&file_data));
            if md5_given == md5_sum {
                println!("Md5 sums: Match.")
            } else {
                println!("Md5 sums: Mismatch");
            }
        }
        None => {}
    }

    match args.sha256 {
        Some(sha256_given) => {
            let sha256_sum = sha256::try_digest(&args.file_path)
                .expect("Error: unable to compute sha256 sum for file {args.file_path}.");
            if sha256_given == sha256_sum {
                println!("SHA256 sums: Match.");
            } else {
                println!("SHA256 sums: Mismatch.");
            }
        }
        None => {}
    }
}
