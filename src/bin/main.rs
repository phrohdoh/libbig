extern crate libbig;
use libbig::BigArchive;

use std::env;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let res = run();
    let code = if let Some(e) = res.err() {
        println!("{:?}", e);
        255
    } else {
        0
    };

    std::process::exit(code);
}

fn run() -> Result<i32, std::io::Error> {
    match env::args().nth(1) {
        Some(path) => {
            let f = try!(File::open(&path));
            let mut br = BufReader::new(f);

            if let Ok(big) = BigArchive::new(&mut br) {
                if let Some(filename) = env::args().nth(2) {
                    if let Some(entry) = big.get_entry(&filename) {
                        println!("{:#?}", entry);
                    } else {
                        println!("{} does not exist in {}", filename, path);
                    }
                } else {
                    println!("{:#?}", big);
                }

                Ok(0)
            } else {
                Ok(2)
            }
        }
        None => {
            println!("Please provide a path to a .big archive");
            println!("");
            println!("Example:");
            println!("  cargo run -- test.big");
            println!("  cargo run -- test.big data/test.ini");
            Ok(1)
        }
    }
}