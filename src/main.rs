use std::rc::Rc;

use hittable::{Hittable, HitRecord};
use ray::Ray;
use vec3::{unit_vector, random_in_unit_sphere, random_unit_vector};

use crate::{
    camera::Camera, color::write_color, hittable::HittableList, sphere::Sphere, vec3::{Vec3, Point, Color}, utils::random_double, lambertian::Lambertian, material::Material, metal::Metal,
};

mod camera;
mod color;
mod hittable;
mod ray;
mod sphere;
mod utils;
mod vec3;
mod material;
mod lambertian;
mod metal;

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
                return attenuation * ray_color(&scattered, world, depth-1);
            }
        }
        return Color::new(0.0,0.0,0.0);
    }
    let unit_direction = unit_vector(ray.direction());
    let t = 0.5*(unit_direction.y() + 1.0);
    return (1.0 - t)*Color::new(1.0, 1.0, 1.0) + t*Color::new(0.5, 0.7, 1.0);
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1000;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;
    // materials
    let matte_green: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let matte_red: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let metal_grey: Rc<dyn Material> = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let metal_blue: Rc<dyn Material> = Rc::new(Metal::new(Color::new(0.6, 0.6, 0.9)));


    // world
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, Rc::clone(&matte_green))));
    world.add(Rc::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5, Rc::clone(&matte_red))));
    world.add(Rc::new(Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.5, Rc::clone(&metal_grey))));
    world.add(Rc::new(Sphere::new(Point::new(1.0, 0.0, -1.0), 0.5, Rc::clone(&metal_blue))));

    let cam = Camera::new();

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
                pixel_color += ray_color(&ray,&world, max_depth);
            }
            write_color(pixel_color, samples_per_pixel);
        }
    }
    eprintln!();
    eprintln!("Done.");
}
