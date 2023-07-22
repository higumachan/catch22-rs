use crate::primitive::Float;

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
