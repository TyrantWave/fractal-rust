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

/// Used for the standard colouring method
pub enum StandardColors {
    SUM,
    REAL,
    IMAGINARY,
}

/// Standard sum colouring of a given vector `input` of results.
///
/// `flag` denotes which part to work on:
///     StandardColors::SUM => Sum of real & imaginary
///     StandardColors::REAL => Scale off the real part only
///     StandardColors::IMAGINARY => Scale off the imaginary part only
pub fn standard_color(input: &Vec<FractalResult>, mode: StandardColors) -> Vec<u8> {
    let output: Vec<u8> = input
        .into_iter()
        .map(|result| match mode {
            StandardColors::SUM => (5.0 * (4.0 + result.value.re + result.value.im)) as u8,
            StandardColors::REAL => (5.0 * (4.0 + result.value.re)) as u8,
            StandardColors::IMAGINARY => (5.0 * (4.0 + result.value.im)) as u8,
        })
        .collect();

    output
}
