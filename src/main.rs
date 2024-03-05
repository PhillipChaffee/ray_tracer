mod vec3;
mod color;
mod ray;
mod hit;
mod sphere;
mod util;
mod interval;

extern crate log;
extern crate env_logger;

use log::{info};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3, unit_vector};
use crate::color::Color;
use crate::hit::{Hittable, HittableList};
use crate::interval::Interval;
use crate::sphere::Sphere;

fn ray_color(ray: &Ray, world: &Box<&dyn Hittable>) -> Color {
    let hit_record = world.hit(ray, Interval::new(0.0, f64::INFINITY));
    if hit_record.hit { return 0.5 * (hit_record.normal.unwrap() + Color::new(1.0, 1.0, 1.0)); }

    let unit_direction = unit_vector(ray.direction());
    let a = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - a) * Color { x: 1.0, y: 1.0, z: 1.0 } + a * Color { x: 0.5, y: 0.7, z: 1.0 };
}

fn main() {
    env_logger::init();

    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // Calculate the image height, and ensure that it is at least 1.
    let mut image_height = (image_width as f64 / aspect_ratio) as i32;
    image_height = if image_height < 1 { 1 } else { image_height };

    // World

    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3 { x: 0.0, y: 0.0, z: 0.0 };

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3 { x: viewport_width, y: 0.0, z: 0.0 };
    let viewport_v = Vec3 { x: 0.0, y: -viewport_height, z: 0.0 };

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center - Vec3 { x: 0.0, y: 0.0, z: focal_length } - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_location = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render

    println!("P3\n{image_width} {image_height}\n255\n");

    for j in 0..image_height {
        let scanlines_remaining = image_height - j;
        info!("\rScanlines remaining: {scanlines_remaining} ");
        for i in 0..image_width {
            let pixel_center = pixel00_location + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&ray, &Box::new(&world));
            color::write_color(pixel_color);
        }
    }

    info!("\rDone\n");
}
