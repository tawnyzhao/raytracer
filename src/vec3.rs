use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

use crate::utils::random_double;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    e: [f64; 3],
}

pub type Point = Vec3;
pub type Color = Vec3;

impl Default for Vec3 {
    fn default() -> Self {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn random(min: f64, max: f64) -> Self {
        Vec3 {
            e: [
                random_double(min, max),
                random_double(min, max),
                random_double(min, max),
            ],
        }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0].powi(2) + self.e[1].powi(2) + self.e[2].powi(2)
    }

    pub fn near_zero(&self) -> bool {
        let epsilon = 1e-8;
        (f64::abs(self.e[0]) < epsilon)
            && (f64::abs(self.e[1]) < epsilon)
            && (f64::abs(self.e[2]) < epsilon)
    }
}

pub fn dot(p1: Vec3, p2: Vec3) -> f64 {
    p1[0] * p2[0] + p1[1] * p2[1] + p1[2] * p2[2]
}

pub fn cross(p1: Vec3, p2: Vec3) -> Vec3 {
    Vec3::new(
        p1.e[1] * p2.e[2] - p1.e[2] * p2.e[1],
        p1.e[2] * p2.e[0] - p1.e[0] * p2.e[2],
        p1.e[0] * p2.e[1] - p1.e[1] * p2.e[0],
    )
}

pub fn unit_vector(p: Vec3) -> Vec3 {
    p / p.length()
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    unit_vector(random_in_unit_sphere())
}

pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if dot(normal, in_unit_sphere) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(random_double(-1.0, 1.0), random_double(-1.0, 1.0), 0.0);

        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot(v, n) * n
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = f64::min(dot(-uv, n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())) * n;
    r_out_perp + r_out_parallel
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.e[0] /= rhs;
        self.e[1] /= rhs;
        self.e[2] /= rhs;
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
            ],
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            e: [
                self.e[0] * rhs.e[0],
                self.e[1] * rhs.e[1],
                self.e[2] * rhs.e[2],
            ],
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        Self {
            e: [self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dimensions() {
        let p = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(p.x(), 1.0);
        assert_eq!(p.y(), 2.0);
        assert_eq!(p.z(), 3.0);
    }

    #[test]
    fn neg() {
        let p1 = Vec3::new(1.0, 2.0, 3.0);
        let p2 = Vec3::new(-1.0, -2.0, -3.0);
        assert_eq!(-p1, p2);
        assert_eq!(p1, -p2);

        let neg_p1 = -p1;
        assert_eq!(neg_p1.x(), -1.0);
        assert_eq!(neg_p1.y(), -2.0);
        assert_eq!(neg_p1.z(), -3.0);
    }

    #[test]
    fn index() {
        let p = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(p[0], 1.0);
        assert_eq!(p[1], 2.0);
        assert_eq!(p[2], 3.0);

        let mut p = p;
        p[0] = 2.0;
        p[1] = 4.0;
        p[2] = 6.0;
        assert_eq!(p[0], 2.0);
        assert_eq!(p[1], 4.0);
        assert_eq!(p[2], 6.0);
    }

    #[test]
    fn assign() {
        let mut p = Vec3::new(1.0, 2.0, 3.0);
        p *= 6.0;
        assert_eq!(p, Vec3::new(6.0, 12.0, 18.0));
        p /= 2.0;
        assert_eq!(p, Vec3::new(3.0, 6.0, 9.0));
        p += Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(p, Vec3::new(4.0, 7.0, 10.0));
        let p2 = Vec3::new(1.0, 1.0, 1.0);
        p += p2;
        assert_eq!(p, Vec3::new(5.0, 8.0, 11.0));
        assert_eq!(p2, Vec3::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn length() {
        let p = Vec3::new(1.0, 0.0, 0.0);
        assert_eq!(p.length(), 1.0);
    }

    #[test]
    fn length_squared() {
        let p = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(p.length_squared(), 1.0 + 4.0 + 9.0);
    }

    #[test]
    fn ops() {
        let p1 = Vec3::new(1.0, 2.0, 3.0);
        let p2 = Vec3::new(0.5, 1.0, 1.5);

        assert_eq!(p1 + p2, Vec3::new(1.5, 3.0, 4.5));
        assert_eq!(p1 - p2, Vec3::new(0.5, 1.0, 1.5));
        assert_eq!(p2 * 2.0, p1);
        assert_eq!(2.0 * p2, p1);
        assert_eq!(p1 / 2.0, p2);
    }

    #[test]
    fn dot_and_cross() {
        let p1 = Vec3::new(1.0, 2.0, 3.0);
        let p2 = Vec3::new(4.0, 5.0, 6.0);

        assert_eq!(dot(p1, p2), 4.0 + 10.0 + 18.0);
        assert_eq!(cross(p1, p2), Vec3::new(-3.0, 6.0, -3.0));
    }
}
