#include <iostream>

#include "color.h"
#include "vec3.h"
int main() {
    const auto image_width = 256;
    const auto image_height = 256;

    std::cout << "P3" << '\n';
    std::cout << image_width << ' ' << image_height << '\n';
    std::cout << 255 << '\n';

    for (auto j = image_height - 1; j >= 0; j--) {
        std::cerr << "\rScanlines remaining: " << j << ' ' << std::flush;
        for (auto i = 0; i < image_width; i++) {
            color pixel_color(double(i) / (image_width - 1),
                              double(j) / (image_height - 1), 0.25);
            write_color(std::cout, pixel_color);
        }
    }
    std::cerr << "\nDone.\n";
}