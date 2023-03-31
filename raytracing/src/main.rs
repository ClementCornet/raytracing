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

mod material;
use material::*;
use material::MaterialTypes::*;

fn main() {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT:u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 50;
    const MAX_DEPTH: u32 = 50;

    // World
    let mut world = HittableList::new();
    let material_ground = Material::new_lambertian(Color::new(0.8,0.8,0.0));
    let material_center = Material::new_lambertian(Color::new(0.7,0.3,0.3));
    let material_left = Material::new_metal(Color::new(0.8,0.8,0.8),0.3);
    let material_right = Material::new_metal(Color::new(0.8,0.6,0.2),1.0);

    world.add(Box::new(Sphere::new(Point3::new(0.0,-100.5,-1.0), 100.0, material_ground)));
    world.add(Box::new(Sphere::new(Point3::new(0.0,0.0,-1.0), 0.5, material_center)));
    world.add(Box::new(Sphere::new(Point3::new(-1.0,0.0,-1.0), 0.5, material_left)));
    world.add(Box::new(Sphere::new(Point3::new(1.0,0.0,-1.0), 0.5, material_right)));

    // Camera
    let cam = Camera::new();

    // Render
    println!("P3\n{} {}\n255",IMAGE_WIDTH,IMAGE_HEIGHT);
    for j in (0..IMAGE_HEIGHT).rev(){
        eprintln!("Scanlines remaining: {}",j); // eprintln! to write to the standard error stream
        for i in 0..IMAGE_WIDTH{


            let mut pixel_color = Color::new(0.0,0.0,0.0);
            for s in 0..SAMPLES_PER_PIXEL{
                let u = (i as f32 + rand_double(0.0, 1.0)) / (IMAGE_WIDTH as f32 - 1.0);
                let v = (j as f32 + rand_double(0.0, 1.0)) / (IMAGE_HEIGHT as f32 - 1.0);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
    eprintln!("Done");

}

fn ray_color(r: &Ray, world: &HittableList, depth: u32) -> Color{

    if depth <= 0{
        return Color::new(0.0,0.0,0.0);
    }

    let mut rec: HitRecord = HitRecord::blank();
    if world.hit(r, 0.001, INFINITY, &mut rec){
        //let target: Point3 = rec.p + rec.normal + Vec3::rand_unit_vector(); // Lambertian Reflection
        ////let target: Point3 = rec.p + Vec3::rand_in_hemisphere(&rec.normal); // Hemispheral Scattering
        //return 0.5 * ray_color(&Ray::new(rec.p, target-rec.p), world, depth-1) // recursive : diffusion
        ////return 0.5 * (rec.normal + Color::new(1.0,1.0,1.0));

        let mut scattered = Ray::new(Point3::zeros(), Vec3::zeros()); //dummy init
        let mut attenuation= Color::new(0.0,0.0,0.0); //dummy init
        if rec.mat.clone().scatter(r, &mut rec, &mut attenuation, &mut scattered){
            return attenuation * ray_color(&scattered, &world, depth-1);
        }
        return Color::new(0.0,0.0,0.0); // Return black
    }

    let unit_direction = unit_vector(&r.dir);
    let t = 0.5 * (unit_direction.y + 1.0);
    return (1.0-t) * Color::new(1.0,1.0,1.0) + t * Color::new(0.5,0.7,1.0);

}

