use crate::primitive::Float;

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
