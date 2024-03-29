use crate::vec3::Vec3;
use crate::vec3::Point3;

pub struct Ray {
    origin: Point3,
    direction: Vec3
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {origin, direction}
    }

    pub fn origin(&self) -> Point3 {
        return self.origin;
    }

    pub fn direction(&self) -> Vec3 {
        return self.direction;
    }

    pub fn at(&self, t: f64) -> Point3 {
        return self.origin + t*self.direction;
    }
}