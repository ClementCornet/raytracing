#![allow(dead_code)]
#![allow(unused_variables)]

use crate::vec3::*;
use crate::hittable::*;
use crate::ray::*;

struct Sphere {
    center: Point3,
    radius: f32
}

impl Sphere {
    fn new(cen: Point3, r: f32) -> Sphere{
        Sphere { center:cen, radius:r }
    }
}

impl Hittable for Sphere {
    fn hit(&self,r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = r.orig - self.center; // Origin-Center Vector
        let a = r.dir.square_length(); // a b c of equation searching for solutions of "r hits sphere"
        let half_b = dot(&oc, &r.dir);
        let c = oc.square_length() - self.radius*self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return false
        }
        let sqrtd = discriminant.sqrt();
        
        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return false
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        return true
    }
}