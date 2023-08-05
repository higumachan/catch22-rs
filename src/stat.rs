use crate::primitive::Float;
use itertools::Itertools;

pub fn median(y: &[Float]) -> Float {
    let mut y = y.to_vec();
    y.sort_by(|x, y| x.partial_cmp(y).unwrap());
    let mid = y.len() / 2;
    if y.len() % 2 == 1 {
        y[mid]
    } else {
        (y[mid - 1] + y[mid]) / 2.0
    }
}

pub fn linear_regression<'a>(
    x: impl ExactSizeIterator<Item = &'a Float> + 'a,
    y: impl ExactSizeIterator<Item = &'a Float> + 'a,
) -> (Float, Float) {
    let len = x.len();
    let (sumx, sumx2, sumxy, sumy) = x.zip_eq(y).fold(
        (0.0, 0.0, 0.0, 0.0),
        |(sumx, sumx2, sumxy, sumy), (x, y)| (sumx + x, sumx2 + x * x, sumxy + x * y, sumy + y),
    );

    let denom = (len as Float) * sumx2 - sumx * sumx;

    if denom == 0.0 {
        return (0.0, 0.0);
    }

    let a = ((len as Float) * sumxy - sumx * sumy) / denom;
    let b = (sumy * sumx2 - sumx * sumxy) / denom;

    (a, b)
}

pub fn linear_regression_and_transform<'a>(
    x: &'a [Float],
    y: &'a [Float],
) -> impl Iterator<Item = Float> + 'a {
    let (a, b) = linear_regression(x.iter(), y.iter());
    x.iter().map(move |&x| a * x + b)
}

pub fn linear_regression_square_error(x: &[Float], y: &[Float]) -> Float {
    let y_hat = linear_regression_and_transform(x, y).collect_vec();
    y.iter()
        .zip_eq(y_hat.iter())
        .map(|(&y, y_hat)| (y - y_hat).powi(2))
        .sum::<Float>()
}
