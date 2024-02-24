use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
}

pub trait Hittable {
    fn hit(&self, ray: Ray, ray_tmin: f64, ray_tmax: f64, hit: &mut HitRecord) -> bool;
}