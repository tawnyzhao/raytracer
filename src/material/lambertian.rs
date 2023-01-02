use crate::{
    material::Material,
    ray::Ray,
    vec3::{random_unit_vector, Color, random_in_unit_sphere, random_in_hemisphere},
};

pub struct Lambertian {
    albedo: Color,
    distribution: Distribution,
}

#[allow(dead_code)]
pub enum Distribution {
    Uniform,
    Sphere,
    Lambertian,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self {
            albedo,
            distribution: Distribution::Lambertian,
        }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _: &crate::ray::Ray,
        rec: &crate::hittable::HitRecord,
    ) -> Option<(Color, Ray)> {

        let mut scatter_direction = match &self.distribution {
            Distribution::Uniform => rec.normal + random_in_hemisphere(rec.normal), 
            Distribution::Sphere => rec.normal + random_in_unit_sphere(),
            Distribution::Lambertian => rec.normal + random_unit_vector(),
        };
        
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}
