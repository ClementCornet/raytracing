#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod vec3;
use vec3::*;

mod color;
use color::*;

mod ray;
use ray::*;

fn main() {
    
    blue_white_gradient();
}

fn test_ppm() {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    println!("P3\n{} {}\n255",IMAGE_WIDTH,IMAGE_HEIGHT);
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

fn ray_color(r: &Ray) -> Color{
    if hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, &r){
        return  Color::new(1.0,0.0,0.0);
    }

    let unit_direction = unit_vector(&r.dir);
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0 - t) * Color::new(1.0,1.0,1.0) + t * Color::new(0.5, 0.7, 1.0);
}

fn blue_white_gradient(){
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT:u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;

    // Camera
    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = ASPECT_RATIO * viewport_height;
    let focal_length: f32 = 1.0;

    let origin: Point3 = Point3::new(0.0, 0.0, 0.0);
    let horizontal: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0,0.0,focal_length);

    // Render
    println!("P3\n{} {}\n255",IMAGE_WIDTH,IMAGE_HEIGHT);
    for j in (0..IMAGE_HEIGHT).rev(){
        eprintln!("Scanlines remaining: {}",j); // eprintln! to write to the standard error stream
        for i in 0..IMAGE_WIDTH{
            let u: f32 = i as f32 / (IMAGE_WIDTH as f32 - 1.0);
            let v: f32 = j as f32 / (IMAGE_HEIGHT as f32 - 1.0);
            let r: Ray = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical - origin);

            let pixel_color =  ray_color(&r);
            write_color(pixel_color);
        }
    }
    eprintln!("Done");
}

fn hit_sphere(center: &Point3, radius: f32, r: &Ray) -> bool {
    // Hard code quadractic equation of wether a ray hits a sphere
    let oc = r.orig - *center;
    let a = dot(&r.dir, &r.dir);
    let b = 2.0 * dot(&oc, &r.dir);
    let c = dot(&oc, &oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    return discriminant > 0.0;
}