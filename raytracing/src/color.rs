#![allow(dead_code)]
#![allow(unused_variables)]

use crate::vec3::*;
use crate::rtweekend::*;

pub fn write_color(pixel_color: Color, samples_per_pixel: u32){
    //println!("{} {} {}",(255.999 * pixel_color.x) as u32,(255.999 *pixel_color.y) as u32,(255.999 *pixel_color.z) as u32);
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    let scale = 1.0 / samples_per_pixel as f32;
    r *= scale;
    g *= scale;
    b *= scale;

    println!("{} {} {}",256.0*clamp(r, 0.0, 0.999),
                        256.0*clamp(g, 0.0, 0.999),
                        256.0*clamp(b, 0.0, 0.999));

}