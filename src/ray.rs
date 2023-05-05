/*
    Seiichi Ariga <seiichi.ariga@gmail.com>
 */

use crate::vec3::{Point3, Vec3};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn origin(&self) -> Point3 {
        self.orig
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    } 

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir*t
    }
}
