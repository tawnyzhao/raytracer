use crate::{ray::Ray, hittable::HitRecord, vec3::Color};

pub mod lambertian;
pub mod metal;
pub mod dielectric;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}