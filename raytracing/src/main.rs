#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod vec3;
use vec3::*;

mod color;
use color::*;

fn main() {
    //let mut test = Vec3::new(2.0,1.2,5.7);
    //let test2 = Vec3::new(3.0,2.2,6.7);
//
    //test += test2;
    //test *= 2.0;
//
    //println!("{:?}", unit_vector(&test));

    test_ppm();
}

fn test_ppm() {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    println!("P3\n{} {}\n255",IMAGE_HEIGHT,IMAGE_WIDTH);
    for j in (0..IMAGE_HEIGHT).rev(){
        eprintln!("Scanlines remaining: {}",j); // eprintln! to write to the standard error stream
        for i in 0..IMAGE_WIDTH{
            let r: f32 = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let g: f32 = j as f32 / (IMAGE_HEIGHT - 1) as f32;
            let b: f32 = 0.25;

            let pixel_color =  Color::new(r,g,b);
            write_color(pixel_color);
        }
    }
    eprintln!("Done");
}