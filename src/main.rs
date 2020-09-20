extern crate clap;
extern crate gif;
extern crate jpeg_decoder as jpeg;
extern crate png;
use clap::{App, Arg};
use std::fs::File;
use std::io::BufReader;
use std::process;

fn main() {
    let matches = App::new("verify image")
        .version("0.1")
        .author("bananaumai <banana.umai@gmail.com>")
        .arg(
            Arg::with_name("format")
                .value_name("IMAGE_FORMAT")
                .required(true)
                .possible_values(&["jpeg", "jpg", "png", "gif"])
                .short("f")
                .long("format")
                .help("Sets a either of jpeg, png, or gif")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("FILE")
                .help("Sets a image file to be verified")
                .required(true)
                .index(1),
        )
        .get_matches();

    let file = matches.value_of("FILE").unwrap();
    let f = match File::open(file) {
        Ok(f) => f,
        Err(err) => {
            println!("Can't open {}; {}", file, err);
            process::exit(1);
        }
    };

    match matches.value_of("format").unwrap() {
        "jpeg" | "jpg" => {
            let mut dec = jpeg::Decoder::new(BufReader::new(f));
            match dec.decode() {
                Ok(_) => println!("OK"),
                Err(e) => {
                    println!("{}", e);
                    process::exit(1)
                }
            }
        }
        "png" => {
            let dec = png::Decoder::new(BufReader::new(f));
            match dec.read_info() {
                Ok(_) => println!("OK"),
                Err(e) => {
                    println!("{}", e);
                    process::exit(1)
                }
            }
        }
        "gif" => {
            let dec = gif::Decoder::new(BufReader::new(f));
            match dec.read_info() {
                Ok(_) => println!("OK"),
                Err(e) => {
                    println!("{}", e);
                    process::exit(1)
                }
            }
        }
        _ => panic!("invalid format"),
    }
}
