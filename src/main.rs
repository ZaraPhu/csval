use clap::Parser;
use sha256::try_digest;
use std::path::Path;

// A program that outputs True if the given file has a sha256 struct equal to the sha56 digest passed into the command
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // The file to be comapred
    #[arg(short, long)]
    file_path: String,

    // The provided sha256 sum
    #[arg(short, long)]
    sha256sum: String,
}

fn main() {
    let args = Args::parse();

    let file_path = Path::new(&args.file_path);
    let computed_sum = try_digest(file_path).expect("Expected a file path.");

    if args.sha256sum == computed_sum {
        println!("Got a match!");
    } else {
        println!("Not a match.");
    }
}
