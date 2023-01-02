use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use raytracer::image::print_ppm;
use raytracer::material::Material;
use raytracer::material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};
use raytracer::ray_color;
use std::sync::Arc;

use raytracer::camera::Camera;
use raytracer::hittable::sphere::Sphere;
use raytracer::hittable::HittableList;
use raytracer::utils::random_double;
use raytracer::vec3::{Color, Point, Vec3};

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material: Arc<dyn Material + Sync + Send> =
        Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let ground = Arc::new(Sphere::new(
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
                    let sphere_material: Arc<dyn Material + Sync + Send> =
                        Arc::new(Lambertian::new(albedo));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random(0.0, 1.0) * Color::random(0.0, 1.0);
                    let fuzz = random_double(0.0, 0.5);
                    let sphere_material: Arc<dyn Material + Sync + Send> =
                        Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material: Arc<dyn Material + Sync + Send> =
                        Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let glass_material: Arc<dyn Material + Sync + Send> = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        Arc::clone(&glass_material),
    )));
    let lambertian_material: Arc<dyn Material + Sync + Send> =
        Arc::new(Lambertian::new(Color::new(0.2, 0.2, 0.8)));
    world.add(Arc::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::clone(&lambertian_material),
    )));
    let metal_material: Arc<dyn Material + Sync + Send> =
        Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        Arc::clone(&metal_material),
    )));

    world
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 500;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 50;
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

    // println!("P3");
    // println!("{image_width} {image_height}");
    // println!("255"); // color depth
    let image: Vec<Color> = (0..image_height)
        .into_par_iter()
        .rev()
        .flat_map(|y| {
            (0..image_width)
                .map(|x| {
                    let color: Vec3 =
                        (0..samples_per_pixel).fold(Color::new(0.0, 0.0, 0.0), |acc, _| {
                            let u = (x as f64 + random_double(0.0, 1.0)) / (image_width) as f64;
                            let v = (y as f64 + random_double(0.0, 1.0)) / (image_height) as f64;
                            let ray = cam.get_ray(u, v);
                            acc + ray_color(&ray, &world, max_depth)
                        });
                    color
                })
                .collect::<Vec<Color>>()
        })
        .collect();

    print_ppm(&image, image_width, image_height, samples_per_pixel);
    // let image: Vec<Color> = (0..image_width * image_height)
    //     .par_iter()
    //     .map(|pidx: usize| {
    //         // Compute image coordinates in camera space.
    //         let i = (pidx % image_width) as f32;
    //         let j = (image_height - 1 - (pidx / image_width)) as f32;
    //         (0..samples_per_pixel).map(Color::zero(), |acc, _| {
    //             let u = (i + random_double(0.0, 1.0)) * rwidth;
    //             let v = (j + random_f32_01()) * rheight;
    //             let r = camera.get_ray(u, v);
    //             ray_color(&r, &world, max_depth)
    //         }) / samples_per_pixel as f32
    //     })
    //     .collect();

    // for j in (0..image_height).rev() {
    //     for i in 0..image_width {
    //         let pixel_color: Color = (0..samples_per_pixel)
    //             .map(|| {
    //                 let u = (i as f64 + random_double(0.0, 1.0)) / ((image_width - 1) as f64);
    //                 let v = (j as f64 + random_double(0.0, 1.0)) / ((image_height - 1) as f64);
    //                 let ray = cam.get_ray(u, v);
    //                 ray_color(&ray, &world, max_depth)
    //             })
    //             .sum();
    //     }
    // }

    // eprint!("\x1b[2K\rScanlines remaining: {j}");
    // eprintln!();
    // eprintln!("Done.");
}