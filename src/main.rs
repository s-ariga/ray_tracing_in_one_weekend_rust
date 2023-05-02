/* Seiichi Ariga <seiichi.ariga@gmail.com> */
/* Ray tracing in one weekend. https://raytracing.github.io/books/RayTracingInOneWeekend.html */
use std::io::Write;
use indicatif::ProgressBar;


fn main() {
    // Image
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    // Render
    let header = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("{}", header);

    let bar = ProgressBar::new(IMAGE_HEIGHT as u64);
    let _ = writeln!(&mut std::io::stderr(), "🖼レイトレーシング中!");

    for j in (0..IMAGE_HEIGHT).rev() {
        bar.inc(1);
        for i in 0.. IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH-1) as f64;
            let g: f64 = j as f64 / (IMAGE_WIDTH-1) as f64;
            let b = 0.25;

            let ir = (255.999 * r) as i32;
            let ig: i32 = (255.999 * g) as i32;
            let ib: i32 = (255.999 * b) as i32;
            
            //let line = format!("{} {} {}\n", ir, ig, ib);
            println!("{} {} {}", ir, ig, ib);
        }
    }
    bar.finish();
}
