/* 
  Seiichi Ariga <seiichi.ariga@gmail.com> 
*/

/* https://raytracing.github.io/ */

/* 3次元ベクトルの実装 */

// 演算子のオーバーロード用trait
use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub v: [f64; 3],
}

impl Vec3 {
    // 要素の取り出し用
    pub fn x(&self) -> f64 {
        self.v[0]
    }
    pub fn y(&self) -> f64 {
        self.v[1]
    }
    pub fn z(&self) -> f64 {
        self.v[2]
    }

    pub fn length(&self) -> f64{
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.v[0].powi(2)+self.v[1].powi(2) + self.v[2].powi(2)
    }

    pub fn dot(&self, other: Vec3) -> f64 {
        self.v[0] * other.v[0] + self.v[1] * other.v[1] + self.v[2] * other.v[2]
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3{v:[self.v[1] * other.v[2] - self.v[2] * other.v[1],
            self.v[2] * other.v[0] - self.v[0] * other.v[2],
            self.v[0] * other.v[1] - self.v[1] * other.v[0]]}
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
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
        if t == 0.0 {
            panic!("Div by zero in Vec3!")
        }

        Vec3 {
            v: [self.v[0] / t, self.v[1] / t, self.v[2] / t],
        }
    }
}

// 名前を変えて、読みやすいようにするためのalias
pub type Point3 = Vec3;
pub type Color = Vec3;
