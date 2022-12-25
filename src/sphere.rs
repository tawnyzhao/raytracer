use std::rc::Rc;

use crate::{vec3::{Point, dot}, hittable::Hittable, material::Material};

pub struct Sphere {
    center: Point,
    radius: f64,
    material: Rc<dyn Material>
}

impl Sphere {
    pub fn new(center: Point, r: f64, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            center: center,
            radius: r,
            material: material
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64, rec: &mut crate::hittable::HitRecord) -> bool {
        let oc = ray.origin() - self.center;
        let a  = ray.direction().length_squared();
        let half_b = dot(oc, ray.direction());
        let c = oc.length_squared() - self.radius.powi(2);
        
        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let mut root = (-half_b - f64::sqrt(discriminant)) / a;
        if root < t_min || root > t_max {
            root = (-half_b + f64::sqrt(discriminant)) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        rec.material = Some(Rc::clone(&self.material));
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);
        true
    }
}
