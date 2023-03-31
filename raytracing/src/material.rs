use std::borrow::Borrow;
use std::borrow::BorrowMut;

use crate::ray::*;
use crate::hittable::*;
use crate::vec3::*;
use crate::rtweekend::*;

#[derive(Clone, Copy)]
pub enum MaterialTypes {
    Lambertian,
    Metal,
    Dielectric
}
#[derive(Clone, Copy)]
pub struct Material {
    mat_type: MaterialTypes,
    albedo: Color,
    fuzz: f32, // For metal fuzziness
    ir: f32 // Index of refraction
}


impl Material{
    pub fn blank() -> Material {
        Material { mat_type: MaterialTypes::Lambertian, albedo: Color::new(0.0,0.0,0.0), fuzz: 0.0, ir: 0.0 }
    }
    pub fn new_lambertian(al: Color) -> Material{
        Material { mat_type: MaterialTypes::Lambertian, albedo: al, fuzz: f32::NAN, ir: f32::NAN }
    }
    pub fn new_metal(al: Color, f: f32) -> Material{
        Material { mat_type: MaterialTypes::Metal, albedo: al, fuzz: f, ir: f32::NAN }
    }
    pub fn new_dielectric(ir: f32) -> Material{
        Material { mat_type: MaterialTypes::Dielectric, albedo: Color::zeros(), fuzz: f32::NAN, ir: ir }
    }
}

pub trait Scatter { 
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool ;
    fn scatter_lambertian(&self, r_in: &Ray, rec: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool ;
    fn scatter_metal(&self, r_in: &Ray, rec: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool ;
    fn scatter_dielectric(&self, r_in: &Ray, rec: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
} 

impl Scatter for Material{
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        match self.mat_type {
            MaterialTypes::Lambertian => self.scatter_lambertian(r_in, rec, attenuation, scattered),
            MaterialTypes::Metal => self.scatter_metal(r_in, rec, attenuation, scattered),
            MaterialTypes::Dielectric => self.scatter_dielectric(r_in, rec, attenuation, scattered)
        }
    }
    fn scatter_lambertian(&self, r_in: &Ray, rec: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + Vec3::rand_unit_vector();
        if Vec3::near_zero(scatter_direction){ // catch directions that lead to zeros/inf
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        return true;
    }
    fn scatter_metal(&self, r_in: &Ray, rec: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = Vec3::reflect(&unit_vector(&r_in.dir), &rec.normal);
        let new_scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::rand_in_unit_sphere());
        *attenuation = self.albedo;
        *scattered = new_scattered;
        return dot(&scattered.dir, &rec.normal) > 0.0;
    }
    fn scatter_dielectric(&self, r_in: &Ray, rec: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio =  match rec.front_face {
            true => 1.0/self.ir,
            false => self.ir            
        };
        let unit_direction = unit_vector(&r_in.dir);
        let refracted = Vec3::refract(&unit_direction, &rec.normal, refraction_ratio);
        //*scattered = Ray::new(rec.p, refracted);
        let cos_theta = dot(&-unit_direction, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3;
        if cannot_refract || reflectance(cos_theta, refraction_ratio) > rand_double(0.0, 1.0){
            direction = Vec3::reflect(&unit_direction, &rec.normal);
        }
        else {
            direction = Vec3::refract(&unit_direction, &rec.normal, refraction_ratio);
        }
        *scattered = Ray::new(rec.p, direction);
        return true;
    }
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0-cosine).powi(5);
}