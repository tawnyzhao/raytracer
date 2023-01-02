use hittable::{Hittable};
use ray::Ray;
use vec3::{Color, unit_vector};

pub mod camera;
pub mod color;
pub mod hittable;
pub mod material;
pub mod ray;
pub mod utils;
pub mod vec3;
pub mod image;

pub fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(ray, 0.001, f64::INFINITY) {
        if let Some(ref mat) = rec.material {
            if let Some((attenuation, scattered)) = mat.scatter(ray, &rec) {
                return attenuation * ray_color(&scattered, world, depth - 1);
            }
        }
        return Color::new(0.0, 0.0, 0.0);
    }
    let unit_direction = unit_vector(ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}
