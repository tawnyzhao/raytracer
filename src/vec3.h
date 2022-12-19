#ifndef VEC3_H
#define VEC3_H

#include <array>
#include <cmath>
#include <iostream>

class vec3 {
   public:
    vec3();
    vec3(double e0, double e1, double e2);

    double x() const;
    double y() const;
    double z() const;

    vec3 operator-() const;
    double operator[](int) const;
    double &operator[](int);

    vec3 &operator+=(const vec3 &v);

    vec3 &operator*=(const double t);

    vec3 &operator/=(const double t);

    double length() const;

    double length_squared() const;

    friend std::ostream &operator<<(std::ostream &out, const vec3 &v);
    friend vec3 operator+(const vec3 &u, const vec3 &v);
    friend vec3 operator-(const vec3 &u, const vec3 &v);
    friend vec3 operator*(const vec3 &u, const vec3 &v);
    friend vec3 operator*(double t, const vec3 &v);
    friend vec3 operator*(const vec3 &v, double t);
    friend vec3 operator/(vec3 v, double t);
    friend double dot(const vec3 &u, const vec3 &v);
    friend vec3 cross(const vec3 &u, const vec3 &v);
    friend vec3 unit_vector(vec3 v);

   private:
    std::array<double, 3> e;
};

using point3 = vec3;
using color = vec3;

#endif