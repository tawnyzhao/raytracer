use camera::Camera;
use hittable::{Hittable, HittableList};
use ray::Ray;
use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use utils::random_double;
use vec3::{unit_vector, Color, Vec3};

pub mod camera;
pub mod color;
pub mod hittable;
pub mod image;
pub mod material;
pub mod ray;
pub mod utils;
pub mod vec3;

pub fn create_image(
    image_height: i32,
    image_width: i32,
    samples_per_pixel: i32,
    max_depth: i32,
    cam: &Camera,
    world: &HittableList,
) -> Vec<Color> {
    (0..image_height)
        .into_par_iter()
        .rev()
        .flat_map(|y| {
            let rows = (0..image_width)
                .map(|x| {
                    let color: Vec3 =
                        (0..samples_per_pixel).fold(Color::new(0.0, 0.0, 0.0), |acc, _| {
                            let u = (x as f64 + random_double(0.0, 1.0)) / (image_width) as f64;
                            let v = (y as f64 + random_double(0.0, 1.0)) / (image_height) as f64;
                            let ray = cam.get_ray(u, v);
                            acc + ray_color(&ray, world, max_depth)
                        });
                    color
                })
                .collect::<Vec<Color>>();
            rows
        })
        .collect()
}
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
