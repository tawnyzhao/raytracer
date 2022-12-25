use crate::{
    material::Material,
    ray::Ray,
    utils::clamp,
    vec3::{random_in_unit_sphere, reflect, unit_vector, Color},
};

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
        attenuation: &mut Color,
        scattered: &mut crate::ray::Ray,
    ) -> bool {
        let reflected = reflect(unit_vector(r_in.direction()), rec.normal);
        scattered.clone_from(&Ray::new(
            rec.p,
            reflected + self.fuzz * random_in_unit_sphere(),
        ));
        attenuation.clone_from(&self.albedo);
        true
    }
}
