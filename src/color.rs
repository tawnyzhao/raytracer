use crate::{vec3::Color, utils::clamp};

pub fn write_color(color: Color, samples_per_pixel: i32) {
    // sqrt -> gamma correction of gamma = 2.0
    let r = f64::sqrt(color.x() / samples_per_pixel as f64);
    let g = f64::sqrt(color.y() / samples_per_pixel as f64);
    let b = f64::sqrt(color.z() / samples_per_pixel as f64);

    let r = (256.0 * clamp(r, 0.0, 0.999)) as i32;
    let g = (256.0 * clamp(g, 0.0, 0.999)) as i32;
    let b = (256.0 * clamp(b, 0.0, 0.999)) as i32;

    println!("{r} {g} {b}")
}