use crate::error::{Catch22Error, Catch22Result};
use crate::primitive::Float;
use crate::stat::median;
use itertools::Itertools;
use num::signum;
use unwrap_ord::UnwrapOrd;

pub fn dn_outliner_include_p_001_mdrmd(y: &[Float]) -> Catch22Result<Float> {
    dn_outliner_include_np_001_mdrmd(y, 1)
}

pub fn dn_outliner_include_n_001_mdrmd(y: &[Float]) -> Catch22Result<Float> {
    dn_outliner_include_np_001_mdrmd(y, -1)
}

const STEP: usize = 100;

fn inc() -> f64 {
    1.0 / STEP as f64
}

fn dn_outliner_include_np_001_mdrmd(y: &[Float], sign: i8) -> Catch22Result<Float> {
    // NaN check
    if y.iter().any(|&x| x.is_nan()) {
        return Ok(Float::NAN);
    }

    let y_first = *y.first().ok_or(Catch22Error::EmptyInput)?;
    if y.iter().skip(1).all(|&x| x == y_first) {
        return Ok(0.0);
    }

    let sign = signum(sign);

    let y_work = y.iter().map(|&x| x * sign as f64).collect_vec();
    let tot = y_work.iter().filter(|&&x| x >= 0.0).count() as f64;

    let max = *y_work.iter().max_by_key(|&&x| UnwrapOrd(x)).unwrap();

    if max < 0.01 {
        return Ok(0.0);
    }

    let threshold = ((max * STEP as f64) + 1.0) as usize;

    let mut fbi = None; // 一番最初に見つけたhigh_value_count == 0のときのj
    let mut mj = 0; // high_value_count - 1が2を超えた最初のj
    let mut ms_dti4 = Vec::with_capacity(threshold);
    for j in 0..threshold {
        let indices = y_work
            .iter()
            .enumerate()
            .filter_map(|(idx, &x)| (x > j as f64 * inc()).then_some((idx + 1) as f64))
            .collect_vec();
        let high_value_count = indices.len();
        let _indices_diff = indices
            .iter()
            .zip(indices.iter().skip(1))
            .map(|(&x, &y)| y - x)
            .collect_vec();

        if high_value_count == 0 && fbi.is_none() {
            fbi = Some(j);
        }
        if (high_value_count - 1) as f64 * (STEP as f64) / tot > 2.0 {
            mj = j;
        }
        ms_dti4.push(median(&indices) / (y.len() as f64 / 2.0) - 1.0);
    }
    let fbi = fbi.unwrap_or(threshold - 1);

    let trim_limit = mj.min(fbi);
    Ok(median(&ms_dti4[0..trim_limit + 1]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utility::load_test_data;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_dn_outliner_include_n_001_mdrmd_same_original() {
        let numbers = load_test_data();

        assert_abs_diff_eq!(
            dn_outliner_include_n_001_mdrmd(&numbers).unwrap(),
            -0.23703703703704,
            epsilon = 1e-12
        );
    }

    #[test]
    fn test_dn_outliner_include_p_001_mdrmd_same_original() {
        let numbers = load_test_data();

        assert_abs_diff_eq!(
            dn_outliner_include_p_001_mdrmd(&numbers).unwrap(),
            0.40740740740741,
            epsilon = 1e-12
        );
    }
}
