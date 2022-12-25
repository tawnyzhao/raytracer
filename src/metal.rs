use crate::{vec3::{Color, unit_vector, reflect}, material::Material, ray::Ray};

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self { Self { albedo } }
}

impl Material for Metal {
    fn scatter(&self, r_in: &crate::ray::Ray, rec: &crate::hittable::HitRecord, attenuation: &mut Color, scattered: &mut crate::ray::Ray) -> bool {
        let reflected = reflect(unit_vector(r_in.direction()), rec.normal);
        scattered.clone_from(&Ray::new(rec.p, reflected));
        attenuation.clone_from(&self.albedo);
        true
    }
}

