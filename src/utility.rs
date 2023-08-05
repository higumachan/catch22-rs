use crate::primitive::Float;
use itertools::Itertools;

use unwrap_ord::UnwrapOrd;

pub fn nextpow2(x: usize) -> Option<usize> {
    let mut y = 1;
    while y < x {
        y = y.checked_mul(2)?;
    }
    Some(y)
}

pub fn any_nan(x: &[Float]) -> bool {
    x.iter().any(|&x| x.is_nan())
}

pub fn mean(x: &[Float]) -> Option<Float> {
    if x.is_empty() {
        return None;
    }
    Some(x.iter().sum::<Float>() / x.len() as Float)
}

pub fn mean_iter(values: impl Iterator<Item = Float>) -> Option<Float> {
    let mut sum = 0.0;
    let mut len = 0;
    for value in values {
        sum += value;
        len += 1;
    }
    if len == 0 {
        None
    } else {
        Some(sum / len as Float)
    }
}

pub fn stddev(values: &[Float]) -> Option<Float> {
    if values.len() < 2 {
        return None;
    }
    let m = mean(values)?;
    Some(
        (values.iter().map(|&x| (x - m).powi(2)).sum::<Float>() / (values.len() - 1) as Float)
            .sqrt(),
    )
}

pub fn linspace(start: Float, end: Float, num_groups: usize) -> impl Iterator<Item = Float> {
    let step_size = (end - start) / (num_groups as Float - 1.0);
    (0..num_groups).map(move |i| start + step_size * (i as Float))
}

pub fn quantile(values: Vec<Float>, quant: Float) -> Float {
    let size = values.len();

    let values = values
        .iter()
        .copied()
        .sorted_by_key(|&x| UnwrapOrd(x))
        .collect_vec();

    let q = 0.5 / size as Float;
    if quant < q {
        return values.first().cloned().unwrap(); // min value
    } else if quant > (1.0 - q) {
        return values.last().cloned().unwrap(); // max value
    }

    let quant_idx = size as Float * quant - 0.5;
    let idx_left = quant_idx.floor() as usize;
    let idx_right = quant_idx.ceil() as usize;
    values[idx_left]
        + (quant_idx - idx_left as Float) * (values[idx_right] - values[idx_left])
            / (idx_right - idx_left) as Float
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Alphabet(usize);

impl Alphabet {
    pub fn from_zero_indexed(value: usize) -> Self {
        Self(value)
    }

    pub fn from_one_indexed(value: usize) -> Self {
        assert!(value > 0);
        Self(value - 1)
    }
}

pub fn sb_coarsegrain(values: &[Float], num_groups: usize) -> Vec<Alphabet> {
    let mut th = linspace(0.0, 1.0, num_groups + 1)
        .map(|x| quantile(values.to_vec(), x))
        .collect_vec();
    th[0] -= 1.0;

    values
        .iter()
        .map(|&value| {
            th.windows(2)
                .enumerate()
                .find_map(|(i, window_values)| {
                    (window_values[0] < value && value <= window_values[1])
                        .then_some(Alphabet::from_zero_indexed(i))
                })
                .unwrap()
        })
        .collect_vec()
}

pub fn entropy(values: &[Float]) -> Float {
    let f = values
        .iter()
        .filter_map(|&x| (x > 0.0).then(|| x * x.ln()))
        .sum::<Float>();

    -f
}
