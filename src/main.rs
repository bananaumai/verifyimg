extern crate jpeg_decoder as jpeg;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::process;

const USAGE: &'static str = "Usage: verifyimg <file>";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("{}", USAGE);
        process::exit(1)
    }

    let file_path = &args[1];
    let f = match File::open(file_path) {
        Ok(f) => f,
        Err(err) => {
            println!("Can't open {}; {}", file_path, err);
            process::exit(1);
        }
    };

    let mut dec = jpeg::Decoder::new(BufReader::new(f));
    match dec.decode() {
        Ok(_) => println!("OK"),
        Err(e) => println!("{}", e),
    }
}
