extern crate image;
use image::imageops;

pub fn crop_image(img_path: &String) {
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
