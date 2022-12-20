#include "sphere.h"

Sphere::Sphere() {}

Sphere::Sphere(Point3 center, double r) : center{center}, radius{r} {}

bool Sphere::hit(const Ray &r, double t_min, double t_max, HitRecord &rec) const {
    Vec3 oc = r.origin() - center;
    auto a = r.direction().length_squared();
    auto half_b = dot(oc, r.direction());
    auto c = oc.length_squared() - radius*radius;

    auto discriminant = half_b * half_b - a*c;
    if (discriminant < 0) {
        return false;
    }

    auto sqrtd = std::sqrt(discriminant);

    auto root = (-half_b - sqrtd) / a;
    if (root < t_min || t_max < root) {
        root = (-half_b + sqrtd) / a; // only calculate second root if closer one is not within bounds
        if (root < t_min || t_max < root)
            return false;
    }

    rec.t = root;
    rec.p = r.at(rec.t);
    auto outward_normal = (rec.p - center) / radius;
    rec.set_face_normal(r, outward_normal);

    return true;
}