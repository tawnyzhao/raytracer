use std::{rc::Rc, sync::Arc};

use crate::{vec3::{Point, dot}, hittable::{Hittable, HitRecord}, material::Material};

pub struct Sphere {
    center: Point,
    radius: f64,
    material: Arc<dyn Material + Sync + Send>
}

impl Sphere {
    pub fn new(center: Point, r: f64, material: Arc<dyn Material + Sync + Send>) -> Sphere {
        Sphere {
            center: center,
            radius: r,
            material: material
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a  = ray.direction().length_squared();
        let half_b = dot(oc, ray.direction());
        let c = oc.length_squared() - self.radius.powi(2);
        
        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let mut root = (-half_b - f64::sqrt(discriminant)) / a;
        if root < t_min || root > t_max {
            root = (-half_b + f64::sqrt(discriminant)) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let t = root;
        let p = ray.at(t);
        let material = Some(Arc::clone(&self.material));
        let outward_normal = (p - self.center) / self.radius;
        Some(HitRecord::new(p, t, material, outward_normal, ray))

    }
}
