use crate::error::{Catch22Error, Catch22Result};
use crate::primitive::Float;
use itertools::Itertools;
use num::ToPrimitive;

fn dn_histogram_mode_5(y: &[Float]) -> Catch22Result<Float> {
    dn_histogram_mode::<5, 6>(y)
}

fn dn_histogram_mode_10(y: &[Float]) -> Catch22Result<Float> {
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
        if count > max_count {
            max_count = count;
            num_maxs = 1;
            out = (histgram.edges[i] + histgram.edges[i + 1]) * 0.5;
        } else if count == max_count {
            num_maxs += 1;
            out += (histgram.edges[i] + histgram.edges[i + 1]) * 0.5;
        }
    }

    Ok(out / num_maxs as Float)
}

struct BinData<const B: usize, const BN: usize> {
    count: [usize; B],
    edges: [Float; BN],
}

fn hist_counts<const B: usize, const BN: usize>(y: &[Float]) -> Catch22Result<BinData<B, BN>> {
    let (&min, &max) = y
        .iter()
        .minmax()
        .into_option()
        .ok_or(Catch22Error::EmptyInput)?;

    let bin_step = (max - min) / B as f64;

    let mut count: [usize; B] = [0usize; B];

    for v in y {
        let bin_index = ((v - min) / bin_step).to_usize().unwrap().max(0).min(B - 1);
        count[bin_index] += 1;
    }

    let edges = (0..BN)
        .into_iter()
        .map(|i| i as Float * bin_step + min)
        .collect_vec()
        .try_into()
        .unwrap();

    return Ok(BinData { edges, count });
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
