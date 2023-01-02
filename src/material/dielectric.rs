use crate::{
    material::Material,
    vec3::{unit_vector, Color, dot}, utils::random_double, ray::Ray,
};

use super::{reflect, refract};

pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            index_of_refraction,
        }
    }
    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &crate::ray::Ray,
        rec: &crate::hittable::HitRecord,
    ) -> Option<(Color, Ray)> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = unit_vector(r_in.direction());
        let cos_theta = f64::min(dot(-unit_direction, rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta.powi(2));

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > random_double(0.0, 1.0) {
            reflect(unit_direction, rec.normal)
        } else {
            refract(unit_direction, rec.normal, refraction_ratio)
        };

        let attenuation = Color::new(1.0, 1.0, 1.0);
        let scattered = Ray::new(rec.p, direction);
        Some((attenuation, scattered))
    }
}
