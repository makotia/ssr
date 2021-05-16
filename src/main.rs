use std::{env, fs};

use ssr::crop_image;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let input_path = match args.first() {
        None => {
            eprintln!("Error: Input file path is not specified. ");
            return;
        }
        Some(s) => s,
    };

    match fs::read_dir(input_path) {
        Ok(v) => {
            for p in v {
                crop_image(p.unwrap().path().to_str().unwrap());
            }
        }
        Err(_) => crop_image(input_path),
    };
}
