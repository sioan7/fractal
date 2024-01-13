use num_complex::Complex;

pub fn generate_fractal() -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let imgx = 800;
    let imgy = 800;

    let scalex = 3.0 / imgx as f64;
    let scaley = 3.0 / imgy as f64;

    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f64) as u8;
        let b = (0.3 * y as f64) as u8;

        let cx = y as f64 * scalex - 1.5;
        let cy = x as f64 * scaley - 1.5;

        let c = Complex::new(-0.4, 0.5);
        let mut z = Complex::new(cx, cy);

        let mut i = 0;
        while i < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            i += 1;
        }

        *pixel = image::Rgb([r, i as u8, b]);
    }

    imgbuf
}
