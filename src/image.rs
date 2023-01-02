use crate::{vec3::Color, color::write_color};

pub fn print_ppm(pixel_colors: &[Color], image_width: i32, image_height: i32, samples_per_pixel: i32) {
    println!("P3");
    println!("{image_width} {image_height}");
    println!("255"); // color depth

    for color in pixel_colors {
        write_color(*color, samples_per_pixel);
    }
}