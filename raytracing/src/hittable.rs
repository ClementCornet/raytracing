#![allow(dead_code)]
#![allow(unused_variables)]

use crate::vec3::*;
use crate::ray::*;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool
}

impl HitRecord {
    // Blank, for temp hit records
    pub fn blank() -> HitRecord {
        HitRecord { p: Vec3::zeros(), normal: Vec3::zeros(), t: -1.0, front_face: false }
    }
}

pub trait Hittable { fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;}

pub trait SetFaceNormal { fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3);}

impl SetFaceNormal for HitRecord {
    // inward/outward normal
    fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&r.dir, outward_normal) < 0.0;
        self.normal  = match self.front_face {
            true => *outward_normal,
            false => -*outward_normal
        }
    }
}