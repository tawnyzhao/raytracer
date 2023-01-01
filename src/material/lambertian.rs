use crate::{material::Material, vec3::{Color, random_unit_vector}, ray::Ray};

pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self { Self { albedo } }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &crate::ray::Ray, rec: &crate::hittable::HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector(); // cos distribution
        // let mut scatter_direction = rec.normal + random_in_unit_sphere(); // cos^3 distribution
        // let mut scatter_direction = rec.normal + random_in_hemisphere(rec.normal); // uniform
        
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        scattered.clone_from(&Ray::new(rec.p, scatter_direction));
        attenuation.clone_from(&self.albedo);
        true
    }
}