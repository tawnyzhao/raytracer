#include <iostream>
#include <cmath>

#include "util.h"
#include "color.h"
#include "hittable_list.h"
#include "sphere.h"
#include "camera.h"

Color ray_color(const Ray& r, const Hittable& world) {
    HitRecord rec;
    if (world.hit(r, 0, infinity, rec)) {
        return 0.5 * (rec.normal + Color(1,1,1));
    }
    Vec3 unit_direction = unit_vector(r.direction());
    auto t = 0.5*(unit_direction.y() + 1.0);
    return (1.0-t)*Color(1.0, 1.0, 1.0) + t*Color(0.5, 0.7, 1.0);
}

int main() {
	// image 
	const auto aspect_ratio = 16.0 / 9.0;
    const auto image_width = 1000;
    const auto image_height = static_cast<int>(image_width / aspect_ratio);
	const auto samples_per_pixel = 100;

	// world
	HittableList world;
	world.add(make_shared<Sphere>(Point3(0,-0.2,-1), 0.5));
	world.add(make_shared<Sphere>(Point3(0,0.2,-0.7), 0.25));
	world.add(make_shared<Sphere>(Point3(0,-100.5,-1), 100));

	// camera
	Camera cam;


    std::cout << "P3" << '\n';
    std::cout << image_width << ' ' << image_height << '\n';
    std::cout << 255 << '\n';

    for (auto j = image_height - 1; j >= 0; j--) {
        std::cerr << "\rScanlines remaining: " << j << ' ' << std::flush;
        for (auto i = 0; i < image_width; i++) {
			Color pixel_color(0,0,0);
			for (auto s = 0; s < samples_per_pixel; s += 1) {
				auto u = (double(i) + random_double()) / (image_width - 1);
				auto v = (double(j) + random_double()) / (image_height - 1);
				auto ray = cam.get_ray(u, v);
				pixel_color += ray_color(ray, world);
			}
			
            write_color(std::cout, pixel_color, samples_per_pixel);
        }
    }
    std::cerr << "\nDone.\n";
}