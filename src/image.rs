use std::io::{self, Write};

use crate::{vec3::Color, utils::clamp};

fn format_color(color: Color, samples_per_pixel: i32) -> (u8, u8, u8) {
    // sqrt -> gamma correction of gamma = 2.0
    let r = f64::sqrt(color.x() / samples_per_pixel as f64);
    let g = f64::sqrt(color.y() / samples_per_pixel as f64);
    let b = f64::sqrt(color.z() / samples_per_pixel as f64);

    let r = (256.0 * clamp(r, 0.0, 0.999)) as u8;
    let g = (256.0 * clamp(g, 0.0, 0.999)) as u8;
    let b = (256.0 * clamp(b, 0.0, 0.999)) as u8;

    (r, g, b)
}

pub fn print_ppm_ascii(pixel_colors: &[Color], image_width: i32, image_height: i32, samples_per_pixel: i32) {
    println!("P3");
    println!("{image_width} {image_height}");
    println!("255"); // color depth

    for color in pixel_colors {
        let (r,g,b) = format_color(*color, samples_per_pixel);
        println!("{r} {g} {b}")
    }
}

pub fn print_ppm_binary(pixel_colors: &[Color], image_width: i32, image_height: i32, samples_per_pixel: i32) {
    println!("P6");
    println!("{image_width} {image_height}");
    println!("255");
    
    let image: Vec<u8> = pixel_colors.iter().flat_map(|color| {
        let (r,g,b) = format_color(*color, samples_per_pixel);
        vec![r,g,b]
    }).collect();

    io::stdout().write_all(&image).unwrap();
}