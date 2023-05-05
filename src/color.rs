/*
Seiichi Ariga <seiichi.ariga@gmail.com>
*/

use crate::vec3::Color;

pub fn write_color(pixcel_color: Color) {
    println!("{} {} {}", 
        (pixcel_color.x() * 255.999) as i32, 
        (pixcel_color.y() * 255.999) as i32,  
        (pixcel_color.z() * 255.999) as i32);
}
