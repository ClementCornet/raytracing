use rand::Rng;

pub const INFINITY: f32 = f32::INFINITY;
pub const PI: f32 = std::f32::consts::PI;

pub fn degrees_to_radians(degrees: f32) -> f32{
    degrees * PI / 180.0
}

pub fn rand_double(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max) // f32 from [min ; max[
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min { return min;}
    if x > max { return max;}
    x
}