#ifndef RAY_H
#define RAY_H

#include "vec3.h"

class Ray {
   public:
    Ray();
    Ray(const Point3& origin, const Vec3& direction);

    Point3 origin() const;
    Vec3 direction() const;

    Point3 at(double t) const;

   private:
    Point3 orig;
    Vec3 dir;
};

#endif