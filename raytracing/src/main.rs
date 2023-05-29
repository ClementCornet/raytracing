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

use image::{RgbImage, ImageBuffer, Rgb};

fn main() {
    // Image
    const ASPECT_RATIO: f32 = 3.0 / 2.0;
    //const ASPECT_RATIO: f32 = 2.0 / 3.0;
    const IMAGE_WIDTH: u32 = 1200;
    const IMAGE_HEIGHT:u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 500; // 500, reduced to render faster
    const MAX_DEPTH: u32 = 50;

    // World
    let world = random_scene();

    // Camera
    let lookfrom = Point3::new(13.0,2.0,3.0);
    let lookat = Point3::new(0.0,0.0,0.0);
    let vup = Vec3::new(0.0,1.0,0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(lookfrom,lookat,vup,20.0,ASPECT_RATIO,aperture,dist_to_focus);
    use rayon::prelude::*;
    // Render PPM
    //println!("P3\n{} {}\n255",IMAGE_WIDTH,IMAGE_HEIGHT);
    //for j in (0..IMAGE_HEIGHT).rev(){
    //    eprintln!("Scanlines remaining: {}",j); // eprintln! to write to the standard error stream
    //    for i in 0..IMAGE_WIDTH{
//
//
    //        let mut pixel_color = Color::new(0.0,0.0,0.0);
    //        for s in 0..SAMPLES_PER_PIXEL{
    //            let u = (i as f32 + rand_double(0.0, 1.0)) / (IMAGE_WIDTH as f32 - 1.0);
    //            let v = (j as f32 + rand_double(0.0, 1.0)) / (IMAGE_HEIGHT as f32 - 1.0);
    //            let r = cam.get_ray(u, v);
    //            pixel_color += ray_color(&r, &world, MAX_DEPTH);
    //        }
    //        write_color(pixel_color, SAMPLES_PER_PIXEL);
    //    }
    //}

    // Render PNG
    //let mut buffer: image::RgbImage = image::ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    //for a in buffer.enumerate_pixels_mut(){
    //    let x = a.0;
    //    let z = IMAGE_HEIGHT -  a.1; // PNG goes bottom to top, our raytracer goes top to bottom
    //    let pixel = a.2;
//
    //    let mut pixel_color = Color::new(0.0,0.0,0.0);
    //    for s in 0..SAMPLES_PER_PIXEL{
    //        let u = (x as f32 + rand_double(0.0, 1.0)) / (IMAGE_WIDTH as f32 - 1.0);
    //        let v = (z as f32 + rand_double(0.0, 1.0)) / (IMAGE_HEIGHT as f32 - 1.0);
    //        let r = cam.get_ray(u, v);
    //        pixel_color += ray_color(&r, &world, MAX_DEPTH);
    //    }
    //    *pixel = Rgb(pix_color(pixel_color, SAMPLES_PER_PIXEL));
    //}  

    //Rrender paraller
    let mut buffer: image::RgbImage = image::ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT); 
    use std::sync::{Arc, Mutex};
    //println!("{:?}", buffer.par_iter());
    //for j in (0..IMAGE_HEIGHT).rev(){
    //    eprintln!("Scanlines remaining: {}",j); // eprintln! to write to the standard error stream
    //    //let mut pixel_color = Color::new(0.0,0.0,0.0);
    //    let pixel_color = Arc::new(Mutex::new(Color::new(0.0,0.0,0.0)));
    //    for i in 0..IMAGE_WIDTH{
    //        (0..SAMPLES_PER_PIXEL).into_par_iter().for_each(|s| {
    //            let u = (i as f32 + rand_double(0.0, 1.0)) / (IMAGE_WIDTH as f32 - 1.0);
    //            let v = (j as f32 + rand_double(0.0, 1.0)) / (IMAGE_HEIGHT as f32 - 1.0);
    //            let r = cam.get_ray(u, v);
    //            //let added = ray_color(&r, &world, MAX_DEPTH);
    //            *pixel_color.lock().unwrap() += ray_color(&r, &world, MAX_DEPTH);
    //            //print!("{:?}", pixel_color)
    //            //sample_one_pixel(s, &mut pixel_color.lock().unwrap(), i, j, IMAGE_WIDTH, IMAGE_HEIGHT, cam, MAX_DEPTH, &world)
    //        });
    //        let col = *pixel_color.lock().unwrap();
    //        let rgbcol = Rgb::from([
    //            (256.0*clamp(col.x, 0.0, 0.999)) as u8, 
    //            (256.0*clamp(col.y, 0.0, 0.999)) as u8, 
    //            (256.0*clamp(col.z, 0.0, 0.999)) as u8]);
    //        buffer.put_pixel(i, j, rgbcol);
//
    //    }
    //}

    //for a in buffer.enumerate_pixels_mut(){
    //    let x = a.0;
    //    let z = IMAGE_HEIGHT -  a.1; // PNG goes bottom to top, our raytracer goes top to bottom
    //    let pixel = a.2;
//
    //    let pixel_color = Arc::new(Mutex::new(Color::new(0.0,0.0,0.0)));
    //    (0..SAMPLES_PER_PIXEL).into_par_iter().for_each(|s| {
    //        let u = (x as f32 + rand_double(0.0, 1.0)) / (IMAGE_WIDTH as f32 - 1.0);
    //        let v = (z as f32 + rand_double(0.0, 1.0)) / (IMAGE_HEIGHT as f32 - 1.0);
    //        let r = cam.get_ray(u, v);
    //        //let added = ray_color(&r, &world, MAX_DEPTH);
    //        *pixel_color.lock().unwrap() += ray_color(&r, &world, MAX_DEPTH);
    //        //print!("{:?}", pixel_color)
    //        //sample_one_pixel(s, &mut pixel_color.lock().unwrap(), i, j, IMAGE_WIDTH, IMAGE_HEIGHT, cam, MAX_DEPTH, &world)
    //    });
    //    *pixel = Rgb(pix_color(*pixel_color.lock().unwrap(), SAMPLES_PER_PIXEL));
    //} 
//
    //for j in (0..IMAGE_HEIGHT).rev(){
    //    eprintln!("Scanlines remaining: {}",j); // eprintln! to write to the standard error stream
    //    //let mut pixel_color = Color::new(0.0,0.0,0.0);
    //    let pixel_color = Arc::new(Mutex::new(Color::new(0.0,0.0,0.0)));
    //    for i in 0..IMAGE_WIDTH{
    //        
//
    //        buffer.put_pixel(i, IMAGE_HEIGHT-j, rgbcol);
    //    }
    //}

    let mut pixels: Vec<Color> = Vec::new();
    let mut coords: Vec<Color> = Vec::new();
    for j in (0..IMAGE_HEIGHT).rev(){ // create vec with pixel coordinates
        let pixel_color = Arc::new(Mutex::new(Color::new(0.0,0.0,0.0)));
        for i in 0..IMAGE_WIDTH{
            pixels.push(Color::new(j as f32, i as f32, 0.0));
            coords.push(Color::new(j as f32, i as f32, 0.0));
        }
    }
    
    pixels.par_iter_mut().for_each(|p| { // Determine color for pixel
        let x = p.x;
        let z = p.y;
        let mut pixel_color = Color::new(0.0,0.0,0.0);
        for s in 0..SAMPLES_PER_PIXEL{
            let u = (z as f32 + rand_double(0.0, 1.0)) / (IMAGE_WIDTH as f32 - 1.0);
            let v = (x as f32 + rand_double(0.0, 1.0)) / (IMAGE_HEIGHT as f32 - 1.0);
            let r = cam.get_ray(u, v);
            //let r = cam.get_ray(v, u);
            pixel_color += ray_color(&r, &world, MAX_DEPTH);
        }
        *p = pixel_color;
    });
    //let mut ii = 0;
    //for a in buffer.enumerate_pixels_mut(){
    //    let x = a.0; // Le sens des index dans ce truc là cette fonction
    //    let y = a.1;
    //    let pixel = a.2;
    //    let idx = x+y*IMAGE_WIDTH;
    //    //let idx = x*IMAGE_WIDTH+y;
    //    let pixel_color = pixels[ii as usize];
    //    ii += 1;
    //    *pixel = Rgb(pix_color(pixel_color, SAMPLES_PER_PIXEL));
    //}
    //for p in pixels{
    //    let x = ii / IMAGE_WIDTH;
    //    let y = ii % IMAGE_HEIGHT;
    //    ii += 1;
//
    //    buffer.put_pixel(IMAGE_HEIGHT - x, y, Rgb(pix_color(p, SAMPLES_PER_PIXEL)));
    //}

    for (c,p) in coords.iter().zip(pixels.iter()){
        let x = c.x as u32;
        let y = c.y as u32;
        buffer.put_pixel(y, IMAGE_HEIGHT - x - 1, Rgb(pix_color(*p, SAMPLES_PER_PIXEL)));
        //buffer.put_pixel(IMAGE_HEIGHT - x - 1, y-1, Rgb(pix_color(*p, SAMPLES_PER_PIXEL)));
    }

    //print!("{:?}",buffer);

    //buffer.pixels();

    buffer.save("imageparparparpar.png").unwrap();

    eprintln!("Done");

}

fn ray_color(r: &Ray, world: &HittableList, depth: u32) -> Color{

    if depth <= 0{
        return Color::new(0.0,0.0,0.0);
    }

    let mut rec: HitRecord = HitRecord::blank();
    if world.hit(r, 0.001, INFINITY, &mut rec){

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

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Material::new_lambertian(Color::new(0.5,0.5,0.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material))
    );

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand_double(0.0, 1.0);
            let center = Point3::new(
                        a as f32 + 0.9 * rand_double(0.0, 1.0),
                        0.2,
                        b as f32 + 0.9 * rand_double(0.0, 1.0));
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                // Diffuse
                let sphere_material : Material;
                if choose_mat < 0.8 {
                    let albedo = Color::rand_vec3(0.0, 1.0) * Color::rand_vec3(0.0, 1.0);
                    sphere_material = Material::new_lambertian(albedo);
                    world.add(Box::new(Sphere::new(center,0.2,sphere_material)));
                }
                else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Color::rand_vec3(0.5, 1.0);
                    let fuzz = rand_double(0.0, 0.5);
                    sphere_material = Material::new_metal(albedo, fuzz);
                    world.add(Box::new(Sphere::new(center,0.2,sphere_material)));
                }
                else {
                    // Glass
                    sphere_material = Material::new_dielectric(1.5);
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)))
                }
            }
        }
    }

    let material_1 = Material::new_dielectric(1.5);
    world.add(Box::new(Sphere::new(Point3::new(0.0,1.0,0.0),1.0, material_1)));

    let material_2 = Material::new_lambertian(Color::new(0.4, 0.2, 0.1));
    world.add(Box::new(Sphere::new(Point3::new(-4.0,1.0,0.0),1.0, material_2)));

    let material_3 = Material::new_metal(Color::new(0.7,0.6,0.5), 0.0);
    world.add(Box::new(Sphere::new(Point3::new(4.0,1.0,0.0),1.0, material_3)));

    return  world;
}


//fn sample_one_pixel(s: u32, pixel_color: &mut Color, i: u32, j: u32, IMAGE_WIDTH: u32,
//    IMAGE_HEIGHT: u32, cam: Camera, MAX_DEPTH: u32, world: &HittableList){
//    let u = (i as f32 + rand_double(0.0, 1.0)) / (IMAGE_WIDTH as f32 - 1.0);
//    let v = (j as f32 + rand_double(0.0, 1.0)) / (IMAGE_HEIGHT as f32 - 1.0);
//    let r = cam.get_ray(u, v);
//    //let added = ray_color(&r, &world, MAX_DEPTH);
//    *pixel_color += ray_color(&r, &world, MAX_DEPTH);
//}