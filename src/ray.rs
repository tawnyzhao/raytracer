use crate::vec3::{Point, Vec3};

#[derive(Debug, Default)]
pub struct Ray {
    origin: Point,
    direction: Vec3
}


impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Self {
        Ray {
            origin: origin,
            direction: direction
        }
    }

    pub fn origin(&self) -> Point {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point {
        return self.origin + t * self.direction
    }
}

mod tests {
    use super::*;

    #[test]
    fn ray() {
        let origin = Point::new(1.0, 1.0, 1.0);
        let direction = Vec3::new(1.0, 2.0, 3.0);
        let r = Ray::new(origin, direction);

        assert_eq!(r.origin(), origin);
        assert_eq!(r.direction(), direction);

        assert_eq!(r.at(0.0), origin);
        assert_eq!(r.at(1.0), Point::new(2.0, 3.0, 4.0));
    }
}