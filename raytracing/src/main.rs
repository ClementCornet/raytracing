#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod vec3;

use vec3::*;

mod color;
use color::*;

mod ray;
use ray::*;

mod hittable;
use hittable::*;

mod sphere;
use sphere::*;

mod hittable_list;
use hittable_list::*;

mod rtweekend;
use rtweekend::*;

mod camera;
use camera::*;

fn main() {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT:u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;

    // World
    let mut world = HittableList::new();
    HittableList::add(&mut world, Box::new(Sphere::new(Point3::new(0.0,0.0,-1.0), 0.5)));
    HittableList::add(&mut world, Box::new(Sphere::new(Point3::new(0.0,-100.5,-1.0), 100.0)));

    // Camera
    let cam = Camera::new();
    //let viewport_height: f32 = 2.0;
    //let viewport_width: f32 = ASPECT_RATIO * viewport_height;
    //let focal_length: f32 = 1.0;
////
    //let origin: Point3 = Point3::new(0.0, 0.0, 0.0);
    //let horizontal: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
    //let vertical: Vec3 = Vec3::new(0.0, viewport_height, 0.0);
    //let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0,0.0,focal_length);

    // Render
    println!("P3\n{} {}\n255",IMAGE_WIDTH,IMAGE_HEIGHT);
    for j in (0..IMAGE_HEIGHT).rev(){
        eprintln!("Scanlines remaining: {}",j); // eprintln! to write to the standard error stream
        for i in 0..IMAGE_WIDTH{
            //let u: f32 = i as f32 / (IMAGE_WIDTH as f32 - 1.0);
            //let v: f32 = j as f32 / (IMAGE_HEIGHT as f32 - 1.0);
            //let r: Ray = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical);

            //let pixel_color =  ray_color(&r, &world);

            let mut pixel_color = Color::new(0.0,0.0,0.0);
            for s in 0..SAMPLES_PER_PIXEL{
                let u = (i as f32 + rand_double(0.0, 1.0)) / (IMAGE_WIDTH as f32 - 1.0);
                let v = (j as f32 + rand_double(0.0, 1.0)) / (IMAGE_HEIGHT as f32 - 1.0);
                let r = cam.get_ray(u, v);
                //let r: Ray = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical);
                pixel_color += ray_color(&r, &world);
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
    eprintln!("Done");

}

fn ray_color(r: &Ray, world: &HittableList) -> Color{

    let mut rec: HitRecord = HitRecord::blank();
    if world.hit(r, 0.0, INFINITY, &mut rec){
        return 0.5 * (rec.normal + Color::new(1.0,1.0,1.0));
    }

    let unit_direction = unit_vector(&r.dir);
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0-t) * Color::new(1.0,1.0,1.0) + t * Color::new(0.5,0.7,1.0);

}

