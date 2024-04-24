use std::sync::Arc;

use crate::{
    hit::{Hit, HitRecord},
    material::Scatter,
    ray::Ray,
    vec::{Point3, Vec3},
};

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Arc<dyn Scatter>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Arc<dyn Scatter>) -> Self {
        Self {
            center,
            radius,
            mat,
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length().powi(2);
        let half_b = oc.dot(r.direction());
        let c = oc.length().powi(2) - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let p = r.at(root);
        let mut rec = HitRecord {
            p,
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: root,
            mat: self.mat.clone(),
            front_face: false,
        };

        rec.set_face_normal(r, (p - self.center) / self.radius);

        Some(rec)
    }
}
