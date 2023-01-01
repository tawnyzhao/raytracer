use hittable::{HitRecord, Hittable};
use ray::Ray;
use vec3::{Color, unit_vector};

pub mod camera;
pub mod color;
pub mod hittable;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod utils;
pub mod vec3;

pub fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    let mut rec: HitRecord = Default::default();
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if world.hit(ray, 0.001, f64::INFINITY, &mut rec) {
        let mut scattered: Ray = Default::default();
        let mut attenuation: Color = Default::default();
        if let Some(ref mat) = rec.material {
            if mat.scatter(ray, &rec, &mut attenuation, &mut scattered) {
                return attenuation * ray_color(&scattered, world, depth - 1);
            }
        }
        return Color::new(0.0, 0.0, 0.0);
    }
    let unit_direction = unit_vector(ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
}
