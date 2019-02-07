use num::complex::Complex;
use std::str::FromStr;

/// Result given from a fractal calculation.
/// `escape`: Iterations needed to escape the function, 0 if it did not escape.
/// `value`: Final z value, whether it escaped or not.
#[derive(Clone, Debug)]
pub struct FractalResult {
    pub escape: u32,
    pub value: Complex<f64>,
}

impl FractalResult {
    pub fn zero() -> Self {
        FractalResult {
            escape: 0,
            value: Complex { re: 0.0, im: 0.0 },
        }
    }
}

/// Flag for the render function to decide which equation to use
#[derive(Copy, Clone)]
pub enum Fractal {
    MANDELBROT,
    JULIA,
    NEWTON,
}

impl FromStr for Fractal {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "mandelbrot" => Ok(Fractal::MANDELBROT),
            "julia" => Ok(Fractal::JULIA),
            "newton" => Ok(Fractal::NEWTON),
            _ => Err(()),
        }
    }
}

impl Fractal {
    /// Given a type of Fractal, calculate the result
    pub fn calculate(&self, c: Complex<f64>, seed: Complex<f64>, limit: u32) -> FractalResult {
        match self {
            Fractal::MANDELBROT => mandelbrot(c, seed, limit),
            Fractal::JULIA => julia(c, seed, limit),
            Fractal::NEWTON => newton(c, seed, limit),
        }
    }
}

/// Try to determine if `c` is in the Mandelbrot set, using
/// at most `limit` iterations.
///
/// Returns a `FractalResult`, with a pair consisting of the escape value and final z value
fn mandelbrot(c: Complex<f64>, seed: Complex<f64>, limit: u32) -> FractalResult {
    let mut z = seed;
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return FractalResult {
                escape: limit - i,
                value: z,
            };
        }
    }

    FractalResult {
        escape: 0,
        value: z,
    }
}

/// Try to determine if `z` is in the Julia set, using
/// at most `limit` iterations.
fn julia(start: Complex<f64>, seed: Complex<f64>, limit: u32) -> FractalResult {
    let mut z = start;
    for i in 0..limit {
        z = z * z + seed;
        if z.norm_sqr() > 4.0 {
            return FractalResult {
                escape: limit - i,
                value: z,
            };
        }
    }

    FractalResult {
        escape: 0,
        value: z,
    }
}

/// Try to determine if `z` is in the Newton set
fn newton(start: Complex<f64>, seed: Complex<f64>, limit: u32) -> FractalResult {
    let pow = 3f64;
    let mut z = start;
    for i in 0..limit {
        let newz = ((pow - 1.0) * z.powf(pow) + seed) / (pow * z.powf(pow - 1.0));
        let bail = newz - z;
        if bail.norm_sqr() <= 0.00001 {
            return FractalResult {
                escape: limit - i,
                value: newz,
            };
        };
        z = newz;
    }

    FractalResult {
        escape: 0,
        value: z,
    }
}

/// Parse the string `s` as a coordinate pair, like `"400x600"` or `"1.0,1.5"`.
///
/// Specifically, `s` should have the form <left><sep><right>, where <sep> is the
/// character given by the `separator` argument, and <left> and <right> are both
/// strings which can be parsed by `T::from_str`.
///
/// If `s` is valid, return `Some(x,y)`, else return `None`.
pub fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

/// Parse a pair of floating-point numbers seperated by a comma as a complex number.
pub fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None,
    }
}

/// Given the row and column of a pixel in the output image, return the
/// corresponding point on the complex plane.
///
/// `bounds` is a pair giving the width and height of the image in pixels.
/// `pixel` is a (column, row) pair indicating a particular pixel in that imagw.
/// The `upper_left` and `lower_right` parameters are points on the complex plane
/// designating the area the image covers.
pub fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_pair() {
        assert_eq!(parse_pair::<i32>("", ','), None);
        assert_eq!(parse_pair::<i32>("10,", ','), None);
        assert_eq!(parse_pair::<i32>(",10", ','), None);
        assert_eq!(parse_pair::<i32>("10,20", 'f'), None);
        assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
        assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
        assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
        assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
    }

    #[test]
    fn test_parse_complex() {
        assert_eq!(
            parse_complex("1.25,-0.0625"),
            Some(Complex {
                re: 1.25,
                im: -0.0625
            })
        );
        assert_eq!(parse_complex(",-0.0625"), None);
    }

    #[test]
    fn text_pixel_to_point() {
        assert_eq!(
            pixel_to_point(
                (100, 100),
                (25, 75),
                Complex { re: -1.0, im: 1.0 },
                Complex { re: 1.0, im: -1.0 }
            ),
            Complex { re: -0.5, im: -0.5 }
        )
    }

}
