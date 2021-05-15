use std::env;

extern crate image;
use image::imageops;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let img_path = match args.first() {
        None => {
            eprintln!("Error: Input file path is not specified. ");
            return;
        }
        Some(s) => s,
    };

    let img = image::open(&img_path).unwrap();
    let mut img = img.to_rgb8();

    let input_height = img.height();
    let input_width = img.width();
    let height = input_width / 16 * 9;

    let res = imageops::crop(
        &mut img,
        0,
        (input_height - height) / 2,
        input_width,
        height,
    );

    match res.to_image().save(img_path) {
        Ok(_) => println!("Succeed."),
        Err(why) => eprintln!("Error!: {:?}", why),
    };
}
