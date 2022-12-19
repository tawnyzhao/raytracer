#ifndef COLOR_H
#define COLOR_H

#include <iostream>

#include "vec3.h"

void write_color(std::ostream &out, Color pixel_color) {
    // 255.999 rounds to 255 which is the max color
    out << static_cast<int>(255.999 * pixel_color.x()) << ' '
        << static_cast<int>(255.999 * pixel_color.y()) << ' '
        << static_cast<int>(255.999 * pixel_color.z()) << '\n';
}

#endif