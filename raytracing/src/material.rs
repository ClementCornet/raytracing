use std::borrow::Borrow;
use std::borrow::BorrowMut;

use crate::ray::*;
use crate::hittable::*;
use crate::vec3::*;
use crate::rtweekend::*;

#[derive(Clone, Copy)]
pub enum MaterialTypes {
    Lambertian,
    Metal
}
#[derive(Clone, Copy)]
pub struct Material {
    mat_type: MaterialTypes,
    albedo: Color,
    fuzz: f32, // For metal fuzziness
}


impl Material{
    pub fn blank() -> Material {
        Material { mat_type: MaterialTypes::Lambertian, albedo: Color::new(0.0,0.0,0.0), fuzz: 0.0 }
    }
    pub fn new_lambertian(al: Color) -> Material{
        Material { mat_type: MaterialTypes::Lambertian, albedo: al, fuzz: 0.0} // metal fuzziness at zero (no importance)
    }
    pub fn new_metal(al: Color, f: f32) -> Material{
        Material { mat_type: MaterialTypes::Metal, albedo: al, fuzz: f }
    }
}

pub trait Scatter { 
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool ;
    fn scatter_lambertian(&self, r_in: &Ray, rec: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool ;
    fn scatter_metal(&self, r_in: &Ray, rec: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool ;
} 

impl Scatter for Material{
    fn scatter(&self, r_in: &Ray, rec: &mut HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        match self.mat_type {
            MaterialTypes::Lambertian => self.scatter_lambertian(r_in, rec, attenuation, scattered),
            MaterialTypes::Metal => self.scatter_metal(r_in, rec, attenuation, scattered)
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
        *scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::rand_in_unit_sphere());
        *attenuation = self.albedo;
        return dot(&scattered.dir, &rec.normal) > 0.0;
    }
}