use crate::vec3::*;

pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3
}

impl Ray{
    fn empty() -> Ray {
        Ray {orig: Point3::zeros(), dir: Vec3::zeros()}
    }

    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {orig: origin, dir: direction}
    }
}

pub trait RayAT { fn at(&self, t: f32) -> Point3;}

impl RayAT for Ray{
    fn at(&self, t: f32) -> Point3 {
        self.orig + self.dir * t
    }
}