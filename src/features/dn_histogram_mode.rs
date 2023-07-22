use crate::error::Catch22Result;
use crate::hist_count::{hist_counts, BinData};
use crate::primitive::Float;

pub fn dn_histogram_mode_5(y: &[Float]) -> Catch22Result<Float> {
    dn_histogram_mode::<5, 6>(y)
}

pub fn dn_histogram_mode_10(y: &[Float]) -> Catch22Result<Float> {
    dn_histogram_mode::<10, 11>(y)
}

fn dn_histogram_mode<const B: usize, const BN: usize>(y: &[Float]) -> Catch22Result<Float> {
    // NaN check
    if y.iter().any(|&x| x.is_nan()) {
        return Ok(Float::NAN);
    }

    let histgram: BinData<B, BN> = hist_counts::<B, BN>(y)?;

    let mut max_count = 0usize;
    let mut num_maxs = 1usize;
    let mut out = 0.0;
    for (i, &count) in histgram.count.iter().enumerate() {
        match count.cmp(&max_count) {
            std::cmp::Ordering::Greater => {
                max_count = count;
                num_maxs = 1;
                out = (histgram.edges[i] + histgram.edges[i + 1]) * 0.5;
            }
            std::cmp::Ordering::Equal => {
                num_maxs += 1;
                out += (histgram.edges[i] + histgram.edges[i + 1]) * 0.5;
            }
            std::cmp::Ordering::Less => {}
        }
    }

    Ok(out / num_maxs as Float)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utility::load_test_data;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_dn_histogram_mode_5() {
        let numbers = load_test_data();

        assert_abs_diff_eq!(
            dn_histogram_mode_5(&numbers).unwrap(),
            -0.61479911484527,
            epsilon = 1e-6,
        );
    }

    #[test]
    fn test_dn_histogram_mode_10() {
        let numbers = load_test_data();

        assert_abs_diff_eq!(
            dn_histogram_mode_10(&numbers).unwrap(),
            -0.78225446555221,
            epsilon = 1e-6,
        );
    }
}
