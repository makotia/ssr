use std::env;

use ssr::crop_image;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let img_path = match args.first() {
        None => {
            eprintln!("Error: Input file path is not specified. ");
            return;
        }
        Some(s) => s,
    };

    crop_image(img_path);
}
