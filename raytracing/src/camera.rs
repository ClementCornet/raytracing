use crate::rtweekend::degrees_to_radians;
use crate::vec3::*;
use crate::ray::*;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32
}

impl Camera {

    pub fn new(lookfrom: Point3, lookat: Point3, vup: Vec3,vfov: f32,
             aspect_ratio: f32, aperture: f32, focus_dist: f32) -> Camera{
        let theta = degrees_to_radians(vfov);
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let w = unit_vector(&(lookfrom-lookat));
        let u = unit_vector(&cross(&vup, &w));
        let v = cross(&w,&u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - focus_dist * w;

        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius
        }

    }
}

pub trait GetRay {fn get_ray(&self, u: f32, v: f32) -> Ray;}

impl GetRay for Camera {
    fn get_ray(&self, s: f32, t: f32) -> Ray {
        //Ray::new(self.origin, self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin)
        let rd = self.lens_radius * Vec3::rand_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(self.origin + offset,
                self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset)
    }
}