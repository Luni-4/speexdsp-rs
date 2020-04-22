extern crate rustfft;
extern crate speexdsp_fft;

use speexdsp_fft::*;

use rustfft::algorithm::Radix4;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use rustfft::FFT;

const EPSILON: f32 = 1e-8;
const N: usize = 2;

#[test]
fn compare_with_rustfft() {
    let mut input: Vec<Complex<f32>> = vec![Complex::zero(); N];
    let mut output: Vec<Complex<f32>> = vec![Complex::zero(); N];

    // Radix4-FFT
    let fft = Radix4::new(N, false);
    fft.process(&mut input, &mut output);

    let drft_lookup = DrftLookup::new(N);

    assert!(drft_lookup
        .trigcache
        .iter()
        .zip(output.iter())
        .all(|(&a, &b)| (a - b.re).abs() < EPSILON));
}
