use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(pixel_color: Color) -> () {
    println!("{}", format!("{} {} {}", (255.99 * pixel_color.x) as u8, (255.99 * pixel_color.y) as u8, (255.99 * pixel_color.z) as u8));
}