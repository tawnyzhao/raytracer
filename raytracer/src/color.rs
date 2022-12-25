use crate::{vec3::Color, utils::clamp};

pub fn write_color(color: Color, samples_per_pixel: i32) {
    let r = color.x() / samples_per_pixel as f64;
    let g = color.y() / samples_per_pixel as f64;
    let b = color.z() / samples_per_pixel as f64;

    let r = (256.0 * clamp(r, 0.0, 0.999)) as i32;
    let g = (256.0 * clamp(g, 0.0, 0.999)) as i32;
    let b = (256.0 * clamp(b, 0.0, 0.999)) as i32;

    println!("{r} {g} {b}")
}