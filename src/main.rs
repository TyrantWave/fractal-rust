use image::png::PNGEncoder;
use image::ColorType;

use std::fs::File;
use std::io::Write;
use std::str::FromStr;

mod fractal;
use fractal::*;

mod coloring;
use coloring::*;

/// Write the given buffer of `pixels`, with dimensions `bounds` into the file `filename`.
fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(
        &pixels,
        bounds.0 as u32,
        bounds.1 as u32,
        ColorType::Gray(8),
    )?;

    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 8 {
        writeln!(
            std::io::stderr(),
            "Usage: fractal FILE METHOD PIXELS UPPERLEFT LOWERRIGHT SEED LIMIT"
        )
        .unwrap();
        writeln!(
            std::io::stderr(),
            "Example: {} mandel.png  mandelbrot 1000x750 -1.20,0.35 -1,0.20 0,0 255",
            args[0]
        )
        .unwrap();
        writeln!(
            std::io::stderr(),
            "Example: {} julia.png julia 1000x750 -1.50,1 1.5,-1 -0.8,0.156 255",
            args[0]
        )
        .unwrap();
        std::process::exit(1);
    }
    let method = Fractal::from_str(&args[2]).expect("error parsing fractal method");
    let bounds = parse_pair(&args[3], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[4]).expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[5]).expect("error parsing lower right corner point");
    let seed = parse_complex(&args[6]).expect("error parsing seeded value");
    let limit = u32::from_str(&args[7]).expect("error parsing limit");

    // Output results we're going to use to render to an image
    let mut results = vec![FractalResult::zero(); bounds.0 * bounds.1];

    // Spawning threads based on available CPUs
    let threads = num_cpus::get();
    let rows_per_band = bounds.1 / threads + 1;
    {
        let bands: Vec<&mut [FractalResult]> =
            results.chunks_mut(rows_per_band * bounds.0).collect();
        match crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left = pixel_to_point(bounds, (0, top), upper_left, lower_right);
                let band_lower_right =
                    pixel_to_point(bounds, (bounds.0, top + height), upper_left, lower_right);

                spawner.spawn(move |_| {
                    render_to_result(
                        band,
                        band_bounds,
                        band_upper_left,
                        band_lower_right,
                        method,
                        seed,
                        limit,
                    );
                });
            }
        }) {
            Err(e) => {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            }
            Ok(_) => (),
        };
    }

    // Convert our results into a pixels array to draw. Just draw the escape value for the default function.
    let pixels: Vec<u8> = results.into_iter().map(|res| res.escape as u8).collect();
    // Alternatively, use a coloring method on a set of the results.
    // let pixels: Vec<u8> = binary_decomposition(&results);
    // let pixels: Vec<u8> = standard_color(&results, StandardColors::SUM);

    write_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}
