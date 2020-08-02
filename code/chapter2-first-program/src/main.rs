use image::{DynamicImage, Rgba};
use image::GenericImageView;
use imageproc::drawing;
use rand::Rng;

    fn main() {
    let src_image = image::open("../res/0000000000.png").expect("failed to open image file");
    let mut rng = rand::thread_rng();
    let mut canvas = drawing::Blend(src_image.to_rgba());
    for _ in 0..50 {
        let x : i32 = rng.gen_range(0, src_image.width() - 1) as i32;
        let y : i32 = rng.gen_range(0, src_image.height() - 1) as i32;
        drawing::draw_cross_mut(&mut canvas, Rgba([0, 255, 255, 128]), x as i32, y as i32);
    }

    let out_img = DynamicImage::ImageRgba8(canvas.0);
    imgshow::imgshow(&out_img);
}
