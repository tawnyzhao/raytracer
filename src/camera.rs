use crate::{
    ray::Ray,
    vec3::{Point, Vec3, unit_vector, cross, random_in_unit_disk},
};

fn degrees_to_radians(degrees: f64) -> f64{
    return degrees * std::f64::consts::PI / 180.0;
}

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,

    #[allow(dead_code)]
    w: Vec3,
    u: Vec3,
    v: Vec3,

    lens_radius: f64,
}

impl Camera {
    pub fn new(
        look_from: Point,
        look_at: Point,
        v_up: Vec3,
        vertical_fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64
    ) -> Self {
        let theta = degrees_to_radians(vertical_fov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        
        let w = unit_vector(look_from - look_at);
        let u = unit_vector(cross(v_up, w));
        let v = cross(w, u);

        let origin = look_from;
        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * v;
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - focus_distance * w;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            w,
            u,
            v,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
