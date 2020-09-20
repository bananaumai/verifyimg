extern crate clap;
extern crate image;
use clap::{App, Arg};
use image::{gif, jpeg, png};
use std::fs::File;
use std::io::BufReader;
use std::process;

const JPEG: &str = "jpeg";
const JPG: &str = "jpg";
const GIF: &str = "gif";
const PNG: &str = "png";

fn main() {
    let matches = App::new("verify image")
        .version("0.1")
        .author("bananaumai <banana.umai@gmail.com>")
        .arg(
            Arg::with_name("format")
                .value_name("IMAGE_FORMAT")
                .required(true)
                .possible_values(&[GIF, JPG, JPEG, PNG])
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
    let r = BufReader::new(f);
    let err = match matches.value_of("format").unwrap() {
        JPEG | JPG => jpeg::JpegDecoder::new(r).err(),
        PNG => png::PngDecoder::new(r).err(),
        GIF => gif::GifDecoder::new(r).err(),
        _ => panic!("invalid format"),
    };
    match err {
        None => println!("OK"),
        Some(e) => {
            println!("{}", e);
            process::exit(1)
        }
    }
}
