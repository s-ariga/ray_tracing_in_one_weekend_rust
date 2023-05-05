/* Seiichi Ariga <seiichi.ariga@gmail.com> */
/* Ray tracing in one weekend. https://raytracing.github.io/books/RayTracingInOneWeekend.html */
pub mod vec3;
pub mod color;
pub mod ray;

use std::io::Write;
use indicatif::ProgressBar;


fn ray_color(r: ray::Ray) -> vec3::Color {
    let unit_direction: vec3::Vec3 = r.direction().unit_vector();
    let t: f64 = 0.5 * (unit_direction.y() + 1.0);
    vec3::Color{v:[1.0, 1.0, 1.0]}*(1.0-t) + vec3::Color{v:[0.5, 0.7, 1.0]} * t
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0/9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64/ASPECT_RATIO) as i32;

    // Camera
    let viewport_height = 2.0; 
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = vec3::Point3{v: [0., 0., 0.]};
    let horizontal = vec3::Vec3{v: [viewport_width, 0., 0.]};
    let vertical = vec3::Vec3{v: [0., viewport_width, 0.]};
    let lower_left_corner = origin - horizontal/2. - vertical/2. - vec3::Vec3{v:[0., 0., focal_length]};

    // Render
    let header = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("{}", header);

    // プログレスバーで進捗を表示
    let bar = ProgressBar::new(IMAGE_HEIGHT as u64);
    // 現在、PPMはSTDOUTに出力するので、メッセージはSTDERRへ
    let _ = writeln!(&mut std::io::stderr(), "🖼レイトレーシング中!");

    for j in (0..IMAGE_HEIGHT).rev() {
        bar.inc(1);
        for i in 0.. IMAGE_WIDTH {
            let u = i as f64/(IMAGE_WIDTH as f64 - 1.);
            let v = j as f64/(IMAGE_HEIGHT as f64 - 1.);
            let r = ray::Ray{orig: origin, dir: lower_left_corner + horizontal*u + vertical*v - origin};
            let pixcel_color: vec3::Color = ray_color(r);
            color::write_color(pixcel_color);
        }
    }
    bar.finish();
}
