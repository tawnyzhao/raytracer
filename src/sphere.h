#ifndef SPHERE_H
#define SPHERE_H

#include "hittable.h"

class Sphere : public Hittable {
   public:
    Sphere();
    Sphere(Point3 center, double r);

    virtual bool hit(const Ray &r, double t_min, double t_max,
                     HitRecord &rec) const override;
    virtual ~Sphere() {};
   private:
    Point3 center;
    double radius;
};

#endif