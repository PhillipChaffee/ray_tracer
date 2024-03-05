use crate::hit::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{dot, Point3};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> HitRecord {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = dot(oc, ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 { return HitRecord::new(None, None, None, None, false); }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return HitRecord::new(None, None, None, None, false);
            }
        }

        let point = ray.at(root);
        let outward_normal = (point - self.center) / self.radius;
        let normal_is_front_facing = dot(ray.direction(), outward_normal) < 0.0;
        let normal = if normal_is_front_facing { outward_normal } else { -outward_normal };
        return HitRecord::new(Some(point), Some(normal), Some(root), Some(normal_is_front_facing), true);
    }
}