#![allow(dead_code)]
#![allow(unused_variables)]

use std::ops;
use crate::rtweekend::*;

#[derive(Debug, Clone, Copy)]
pub struct Vec3{
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub fn zeros() -> Vec3 {
        Vec3 {x:0.0,y:0.0,z:0.0}
    }
    pub fn new(x:f32, y:f32, z:f32) -> Vec3{
        Vec3 {x:x,y:y,z:z}
    }
    pub fn rand_vec3(min: f32, max: f32) -> Vec3{
        Vec3::new(rand_double(min, max), rand_double(min, max), rand_double(min, max))
    }
    pub fn rand_in_unit_sphere() -> Vec3{
        loop {
            let p = Vec3::rand_vec3(-1.0, 1.0); // Might be in unary cube but not unary sphere
            if p.square_length() >= 1.0 { continue;}
            return p;
        }
    }
    pub fn rand_unit_vector() -> Vec3 {
        unit_vector(&Vec3::rand_in_unit_sphere())
    }
    pub fn rand_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::rand_in_unit_sphere();
        if dot(&in_unit_sphere,normal) > 0.0 {
            return in_unit_sphere;
        }
        return  -in_unit_sphere;
    }
    pub fn near_zero(v: Vec3) -> bool {
        let s: f32 = 1e-8;
        v.x.abs() < s && v.y.abs() < s && v.z.abs() < s
    }
    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        let d = 2.0*dot(v,n);
        *v - *n * d
    }
    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3{
        let cos_theta = dot(&-*uv, n).min(1.0);
        let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
        let r_out_parallel = -(1.0 - r_out_perp.square_length()).abs().sqrt() * *n;
        return r_out_perp + r_out_parallel;
    }

}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl ops::Index<u32> for Vec3 {
    type Output = f32;
    fn index(&self, index: u32) -> &Self::Output {
        match index{
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds. Must be within 0..=2")
        }
    }
}

impl ops::IndexMut<u32> for Vec3 {
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        match index{
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds. Must be within 0..=2")
        }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

pub trait SquaredLength { fn square_length(&self) -> f32;}
pub trait Length { fn length(&self) -> f32;}

impl SquaredLength for Vec3{
    fn square_length(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl Length for Vec3{
    fn length(&self) -> f32 {
        self.square_length().sqrt()
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl ops::Add<Vec3> for Vec3{
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl ops::Sub<Vec3> for Vec3{
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl ops::Mul<Vec3> for Vec3{
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Mul<f32> for Vec3{
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl ops::Mul<Vec3> for f32{
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z
        }
    }
}

impl ops::Div<f32> for Vec3{
    type Output = Vec3;
    fn div(self, rhs: f32) -> Self::Output {
        (1.0/rhs) * self
    }
}

pub fn dot(v1: &Vec3, v2: &Vec3) -> f32{
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3{
    Vec3{
        x: v1.y * v2.z - v1.z * v2.y,
        y: v1.z * v2.x - v1.x * v2.z,
        z: v1.x * v2.y - v1.y * v2.x
    }
}

pub fn unit_vector(v1: &Vec3) -> Vec3{
    *v1 / v1.length()
}