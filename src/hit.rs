use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct HitRecord {
    pub point: Option<Point3>,
    pub normal: Option<Vec3>,
    pub t: Option<f64>,
    pub front_facing: Option<bool>,
    pub hit: bool,
}

impl HitRecord {
    pub fn new(point: Option<Point3>, normal: Option<Vec3>, t: Option<f64>, front_facing: Option<bool>, hit: bool) -> HitRecord {
        HitRecord { point, normal, t, front_facing, hit }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> HitRecord;
}

pub struct HittableList {
    hittables: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self { Self { hittables: vec!() } }
    pub fn clear(&mut self) -> () {
        self.hittables.clear();
    }
    pub fn add(&mut self, hittable: Box<dyn Hittable>) -> () {
        self.hittables.push(hittable);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> HitRecord {
        let mut closest_so_far = ray_t.max;
        let mut hit_record = HitRecord::new(None, None, None, None, false);

        for hittable in &self.hittables {
            let current_rec = hittable.hit(ray, Interval::new(ray_t.min, closest_so_far));
            if current_rec.hit {
                match current_rec.t {
                    Some(t) => closest_so_far = t,
                    None => (),
                }
                hit_record = current_rec;
            }
        }

        return hit_record;
    }
}