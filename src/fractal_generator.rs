use num_complex::Complex;

pub struct FractalProps {
    pub width: u32,
    pub height: u32,
    pub scalex: f64,
    pub scaley: f64,
    pub cx: f64,
    pub cy: f64,
    pub c: Complex<f64>,
}

pub struct FractalGenerator;

impl FractalGenerator {
    pub fn generate_fractal(props: &FractalProps) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let FractalProps {
            width,
            height,
            scalex,
            scaley,
            cx,
            cy,
            c,
        } = *props;
        let mut imgbuf = image::ImageBuffer::new(width, height);

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let r = (0.3 * x as f64) as u8;
            let b = (0.3 * y as f64) as u8;

            let cx = y as f64 * scalex - cx;
            let cy = x as f64 * scaley - cy;

            let mut z = Complex::new(cx, cy);

            let mut i = 0u8;
            while i < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }

            *pixel = image::Rgb([r, i, b]);
        }

        imgbuf
    }
}
