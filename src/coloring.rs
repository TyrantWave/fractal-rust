use super::fractal::FractalResult;

/// Gathers a binary decomposition colouring of a given vector `input` of results.
///
/// Multiplies the real & imaginary parts of the complex value, and colours depending on if it's a positive or negative result
pub fn binary_decomposition(input: &Vec<FractalResult>) -> Vec<u8> {
    let output: Vec<u8> = input
        .into_iter()
        .map(|result| match result.value {
            n if n.re * n.im >= 0.0 => std::u8::MAX,
            _ => 0,
        })
        .collect();

    output
}
