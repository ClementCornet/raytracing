use crate::rtweekend::degrees_to_radians;
use crate::vec3::*;
use crate::ray::*;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,

    //aspect_ratio: f32,
    //viewport_height: f32,
    //viewport_width: f32,
    //focal_length: f32
}

impl Camera {

    pub fn new_oldversion() -> Camera{
        let default_aspect_ratio = 16.0 / 9.0;
        let default_viewport_height = 2.0;
        let default_viewport_width = default_aspect_ratio * default_viewport_height;
        let default_focal_length = 1.0;
        
        let default_origin = Point3::new(0.0,0.0,0.0);
        let default_horizontal = Vec3::new(default_viewport_width, 0.0, 0.0);
        let default_vertical = Vec3::new(0.0,default_viewport_height,0.0);
        let default_lower_left_corner = default_origin - default_horizontal/2.0 - default_vertical/2.0 -
                                                 Vec3::new(0.0,0.0,default_focal_length);

        Camera {
            // CHANGER CA SI ON DOIT UTILISER DES CAMERAS DIFFERENTES
            //aspect_ratio: default_aspect_ratio,
            //viewport_height: default_viewport_height,
            //viewport_width: default_viewport_width , 
            //focal_length: default_focal_length,

            origin: default_origin,
            horizontal: default_horizontal, // (V_W,0,0)
            vertical: default_vertical,
            lower_left_corner: default_lower_left_corner
        }
    }

    pub fn new(lookfrom: Point3, lookat: Point3, vup: Vec3,vfov: f32, aspect_ratio: f32) -> Camera{
        let theta = degrees_to_radians(vfov);
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let w = unit_vector(&(lookfrom-lookat));
        let u = unit_vector(&cross(&vup, &w));
        let v = cross(&w,&u);

        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - w;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical
        }

    }
}

pub trait GetRay {fn get_ray(&self, u: f32, v: f32) -> Ray;}

impl GetRay for Camera {
    fn get_ray(&self, s: f32, t: f32) -> Ray {
        //Ray::new(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin)
        Ray::new(self.origin, self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin)
    }
}