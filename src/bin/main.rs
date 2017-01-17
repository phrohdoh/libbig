extern crate libbig;
use std::env;
use std::fs::File;
use std::io::BufReader;

fn main() {
    std::process::exit(run().unwrap_or(255));
}

fn run() -> Result<i32, std::io::Error> {
    match env::args().nth(1) {
        Some(path) => {
            let f = try!(File::open(path));
            let mut br = BufReader::new(f);

            if let Ok(big) = libbig::BigArchive::new(&mut br) {
                println!("{:#?}", big);
                Ok(0)
            } else {
                Ok(2)
            }
        }
        None => {
            println!("Please provide a path to a .big archive");
            Ok(1)
        }
    }
}