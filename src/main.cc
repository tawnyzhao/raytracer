#include <iostream>
#include <cmath>

#include "color.h"
#include "vec3.h"
#include "ray.h"

// hello world
// int main() {
//     const auto image_width = 256;
//     const auto image_height = 256;

//     std::cout << "P3" << '\n';
//     std::cout << image_width << ' ' << image_height << '\n';
//     std::cout << 255 << '\n';

//     for (auto j = image_height - 1; j >= 0; j--) {
//         std::cerr << "\rScanlines remaining: " << j << ' ' << std::flush;
//         for (auto i = 0; i < image_width; i++) {
//             Color pixel_color(double(i) / (image_width - 1),
//                               double(j) / (image_height - 1), 0.25);
//             write_color(std::cout, pixel_color);
//         }
//     }
//     std::cerr << "\nDone.\n";
// }
double hit_sphere(const Point3 &center, double radius, const Ray &r) {
	Vec3 oc = r.origin() - center;
	auto a = r.direction().length_squared();
	auto half_b = dot(oc, r.direction());
	auto c = oc.length_squared() - radius * radius;

	auto discriminant = half_b*half_b - a*c;
	if (discriminant < 0) {
		return -1.0;
	} else { // return the closest hit point
		return (-half_b - std::sqrt(discriminant)) / a;
	}
	return (discriminant >= 0);
}

Color ray_color(const Ray& r) {
	// sphere
	auto circle_center = Point3(0,0,-1);
	auto circle_radius = 0.5;
	auto t = hit_sphere(circle_center, circle_radius, r);

	if (t > 0.0) {
		Vec3 N = unit_vector(r.at(t) - circle_center);
		return 0.5*Color(N.x() + 1, N.y() + 1, N.z() + 1);  // coordinates is originally from -1 to 1, convert to 0 to 1
	}

	// background
	Vec3 unit_direction = unit_vector(r.direction());
	t = 0.5*(unit_direction.y() + 1.0); // y is originally from -1 to 1, convert to 0 to 1
	return (1.0 - t) * Color(0.6, 0.2, 0.3) + t * Color(0.1, 0.3, 0.7);
}

int main() {
	// image 
	const auto aspect_ratio = 16.0 / 9.0;
    const auto image_width = 1000;
    const auto image_height = static_cast<int>(image_width / aspect_ratio);

	// camera

	auto viewport_height = 2.0;
	auto viewport_width = aspect_ratio * viewport_height;
	auto focal_length = 1.0;

	auto origin = Point3(0,0,0);
	auto horizontal = Vec3(viewport_width, 0, 0);
	auto vertical = Vec3(0, viewport_height, 0);
	auto lower_left_corner = origin - horizontal/2 - vertical/2 - Vec3(0,0, focal_length);


    std::cout << "P3" << '\n';
    std::cout << image_width << ' ' << image_height << '\n';
    std::cout << 255 << '\n';

    for (auto j = image_height - 1; j >= 0; j--) {
        std::cerr << "\rScanlines remaining: " << j << ' ' << std::flush;
        for (auto i = 0; i < image_width; i++) {
			auto u = double(i) / (image_width - 1);
			auto v = double(j) / (image_height - 1);
			auto direction = lower_left_corner + u*horizontal + v*vertical - origin;
			
			Ray r{origin, direction};
            Color pixel_color = ray_color(r);
            write_color(std::cout, pixel_color);
        }
    }
    std::cerr << "\nDone.\n";
}