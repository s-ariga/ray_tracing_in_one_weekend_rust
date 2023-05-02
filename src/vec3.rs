/* Seiichi Ariga <seiichi.ariga@gmail.com> */

/* 3次元ベクトル */

use std::f64::sqrt;
use std::ops::Pow;
// 演算子のオーバーロード用trait
use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, PartialEq)]
pub struct Vec3 {
    v: [f64; 3],
}

impl Vec3 {
    fn length(&self) -> f64{
        sqrt(self.length_squared())
    }

    fn length_squared(&self) -> f64 {
        self.v[0].powi(2)+self.v[1].powi(2) + self.v[2].powi(2)
    }
}

/* Vec3に対して、演算子をオーバーロードしていく */
impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 { 
            v: [
                self.v[0] + other.v[0],
                self.v[1] + other.v[1], 
                self.v[2] + other.v[2]
            ],
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 { 
            v: [
                self.v[0] - other.v[0],
                self.v[1] - other.v[1], 
                self.v[2] - other.v[2]
            ],
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3 {
        Vec3 {
            v: [self.v[0] * t, self.v[1] * t, self.v[2] * t],
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            v: [
                self.v[0] * other.v[0],
                self.v[1] * other.v[1], 
                self.v[2] * other.v[2]
            ],
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Vec3 {
        // ここでゼロ割チェックをしておく
        if t == 0 {
            panic!("Div by zero in Vec3!")
        }

        Vec3 {
            v: [self.v[0] / t, self.v[1] / t, self.v[2] / t],
        }
    }
}

type Point3 = Vec3;
type Color = Vec3;
