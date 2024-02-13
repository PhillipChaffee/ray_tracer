mod vec3;
mod color;

extern crate log;
extern crate env_logger;

use log::{info};

fn main() {
    env_logger::init();

    // Image

    let image_width = 256;
    let image_height = 256;

    // Render

    println!("P3\n{image_width} {image_height}\n255\n");

    for i in 0..image_height {
        let scanlines_remaining = image_height-i;
        info!("\rScanlines remaining: {scanlines_remaining} ");
        for j in 0..image_width {
            let pixel_color = color::Color{
                x: j as f64 / (image_width-1) as f64,
                y: i as f64 / (image_height-1) as f64,
                z: 0.0
            };

            color::write_color(pixel_color);
        }
    }

    info!("\rDone\n");
}
