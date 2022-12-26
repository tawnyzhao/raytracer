use std::rc::Rc;

use hittable::{HitRecord, Hittable};
use ray::Ray;
use vec3::{random_in_unit_sphere, random_unit_vector, unit_vector};

use crate::{
    camera::Camera,
    color::write_color,
    dielectric::Dielectric,
    hittable::HittableList,
    lambertian::Lambertian,
    material::Material,
    metal::Metal,
    sphere::Sphere,
    utils::random_double,
    vec3::{Color, Point, Vec3},
};

mod camera;
mod color;
mod dielectric;
mod hittable;
mod lambertian;
mod material;
mod metal;
mod ray;
mod sphere;
mod utils;
mod vec3;

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Color {
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

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1000;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 200;
    let max_depth = 50;
    // materials
    let matte_green: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let matte_purple: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.7)));
    let metal_grey: Rc<dyn Material> = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.05));
    let metal_blue: Rc<dyn Material> = Rc::new(Metal::new(Color::new(0.2, 0.2, 0.7), 1.0));
    let glass: Rc<dyn Material> = Rc::new(Dielectric::new(1.5));
    // world
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        Rc::clone(&matte_green),
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, 0.0, -1.0),
        0.5,
        Rc::clone(&matte_purple),
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        0.5,
        Rc::clone(&metal_grey),
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(1.0, 0.0, -1.0),
        0.5,
        Rc::clone(&glass),
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(1.0, 1.0, -3.0),
        1.0,
        Rc::clone(&metal_blue),
    )));

    let cam = Camera::new(
        Point::new(-2.0, 2.0, 1.0),
        Point::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        30.0,
        aspect_ratio,
    );

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255"); // color depth

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {j}");
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for s in 0..samples_per_pixel {
                let u = (i as f64 + random_double(0.0, 1.0)) / ((image_width - 1) as f64);
                let v = (j as f64 + random_double(0.0, 1.0)) / ((image_height - 1) as f64);
                let ray = cam.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, max_depth);
            }
            write_color(pixel_color, samples_per_pixel);
        }
    }
    eprintln!();
    eprintln!("Done.");
}
