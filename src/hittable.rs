use std::{sync::Arc};

pub mod sphere;

use crate::{
    material::Material,
    ray::{Ray},
    vec3::{dot, Point, Vec3},
};

#[derive(Default, Clone)]
pub struct HitRecord {
    pub p: Point,
    pub t: f64,
    pub material: Option<Arc<dyn Material + Sync + Send>>,

    pub normal: Vec3,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        p: Point,
        t: f64,
        material: Option<Arc<dyn Material + Sync + Send>>,
        outward_normal: Vec3,
        ray: &Ray
    ) -> Self {
        let (front_face, normal) = Self::get_face_normal(ray, outward_normal);
        Self {
            p,
            t,
            material,
            normal,
            front_face,
        }
    }

    fn get_face_normal(ray: &Ray, outward_normal: Vec3) -> (bool, Vec3) {
        let front_face = dot(ray.direction(), outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        (front_face, normal)
    }
}

pub trait Hittable {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable + Sync + Send>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { objects: vec![] }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable + Sync + Send>) {
        self.objects.push(object);
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut res = None;

        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(temp_rec) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
                res = Some(temp_rec);
            }
        }
        res
    }
}
