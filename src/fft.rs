use crate::primitive::Float;
use itertools::Itertools;
use num::Complex;
use std::f64::consts::PI;

pub fn fft(values: &mut [Complex<Float>], step: usize, tw: &[Complex<Float>]) {
    let mut working = values.to_vec();
    _fft(values, &mut working, values.len(), step, tw);
}

fn _fft(
    values: &mut [Complex<Float>],
    working: &mut [Complex<Float>],
    size: usize,
    step: usize,
    tw: &[Complex<Float>],
) {
    if step < values.len() {
        _fft(working, values, size, step * 2, tw);
        _fft(
            &mut working[step..],
            &mut values[step..],
            size,
            step * 2,
            tw,
        );

        for i in (0..size).step_by(step * 2) {
            let temp = tw[i] * working[i + step];
            values[i / 2] = working[i] + temp;
            values[(i + size) / 2] = working[i] - temp;
        }
    }
}

pub fn twiddles(size: usize) -> Vec<Complex<Float>> {
    (0..size)
        .map(|i| Complex::new(0.0, -PI * (i as Float / size as Float)).exp())
        .collect_vec()
}
