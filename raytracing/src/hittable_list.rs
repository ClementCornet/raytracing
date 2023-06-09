use crate::hittable::*;

pub struct HittableList{
    objects: Vec<Box<dyn Hittable>>
}

impl HittableList{
    pub fn new() -> HittableList {
        HittableList { objects: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>){
        self.objects.push(object);
    }

}

unsafe impl Sync for HittableList {}

impl Hittable for HittableList{
    fn hit(&self, r: &crate::ray::Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord::blank();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter(){
            if object.hit(r, t_min, closest_so_far, &mut temp_rec){
                hit_anything = true;
                closest_so_far = temp_rec.t;
                
            }
        }
        *rec = temp_rec;
        return hit_anything;
    }
}