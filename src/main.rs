use raytracer::material::Material;
use raytracer::material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};
use raytracer::ray_color;
use std::rc::Rc;

use raytracer::camera::Camera;
use raytracer::color::write_color;
use raytracer::hittable::HittableList;
use raytracer::sphere::Sphere;
use raytracer::utils::random_double;
use raytracer::vec3::{Color, Point, Vec3};


fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let ground = Rc::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));
    world.add(ground);
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double(0.0, 1.0);
            let center = Point::new(
                a as f64 + 0.9 * random_double(0.0, 1.0),
                0.2,
                b as f64 + 0.9 * random_double(0.0, 1.0),
            );

            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random(0.0, 1.0) * Color::random(0.0, 1.0);
                    let sphere_material: Rc<dyn Material> = Rc::new(Lambertian::new(albedo));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random(0.0, 1.0) * Color::random(0.0, 1.0);
                    let fuzz = random_double(0.0, 0.5);
                    let sphere_material: Rc<dyn Material> = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material: Rc<dyn Material> = Rc::new(Dielectric::new(1.5));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let glass_material: Rc<dyn Material> = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        Rc::clone(&glass_material),
    )));
    let lambertian_material: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.2, 0.2, 0.8)));
    world.add(Rc::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        Rc::clone(&lambertian_material),
    )));
    let metal_material: Rc<dyn Material> = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        Rc::clone(&metal_material),
    )));

    world
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1600;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 700;
    let max_depth = 50;

    // world
    let world = random_scene();

    let cam = Camera::new(
        Point::new(13.0, 2.0, 3.0),
        Point::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.1,
        10.0,
    );

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255"); // color depth

    for j in (0..image_height).rev() {
        eprint!("\x1b[2K\rScanlines remaining: {j}");
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
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
