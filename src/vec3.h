#ifndef VEC3_H
#define VEC3_H

#include <array>
#include <cmath>
#include <iostream>



class Vec3 {
   public:
    Vec3();
    Vec3(double e0, double e1, double e2);

    double x() const;
    double y() const;
    double z() const;

    Vec3 operator-() const;
    double operator[](int) const;
    double &operator[](int);

    Vec3 &operator+=(const Vec3 &v);

    Vec3 &operator*=(const double t);

    Vec3 &operator/=(const double t);

    double length() const;

    double length_squared() const;

    friend std::ostream &operator<<(std::ostream &out, const Vec3 &v);
    friend Vec3 operator+(const Vec3 &u, const Vec3 &v);
    friend Vec3 operator-(const Vec3 &u, const Vec3 &v);
    friend Vec3 operator*(const Vec3 &u, const Vec3 &v);
    friend Vec3 operator*(double t, const Vec3 &v);
    friend Vec3 operator*(const Vec3 &v, double t);
    friend Vec3 operator/(Vec3 v, double t);
    friend double dot(const Vec3 &u, const Vec3 &v);
    friend Vec3 cross(const Vec3 &u, const Vec3 &v);
    friend Vec3 unit_vector(Vec3 v);

   private:
    std::array<double, 3> e;
};

std::ostream &operator<<(std::ostream &out, const Vec3 &v);
Vec3 operator+(const Vec3 &u, const Vec3 &v);
Vec3 operator-(const Vec3 &u, const Vec3 &v);
Vec3 operator*(const Vec3 &u, const Vec3 &v);
Vec3 operator*(double t, const Vec3 &v);
Vec3 operator*(const Vec3 &v, double t);
Vec3 operator/(Vec3 v, double t);
double dot(const Vec3 &u, const Vec3 &v);
Vec3 cross(const Vec3 &u, const Vec3 &v);
Vec3 unit_vector(Vec3 v);


using Point3 = Vec3;
using Color = Vec3;

#endif