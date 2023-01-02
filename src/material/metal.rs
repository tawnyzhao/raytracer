use crate::{
    material::Material,
    ray::Ray,
    utils::clamp,
    vec3::{random_in_unit_sphere, unit_vector, Color, dot},
};

use super::reflect;

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: clamp(fuzz, 0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &crate::ray::Ray,
        rec: &crate::hittable::HitRecord,
    ) -> Option<(Color, Ray)> {
        let reflected = reflect(unit_vector(r_in.direction()), rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * random_in_unit_sphere(),
        );
        let attenuation = self.albedo;
        if dot(scattered.direction(), rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}
