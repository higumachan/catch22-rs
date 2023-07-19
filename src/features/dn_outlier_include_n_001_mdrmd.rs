use itertools::Itertools;
use num::signum;
use crate::error::{Catch22Error, Catch22Result};
use crate::primitive::Float;
use crate::stat::median;

fn dn_outliner_include_p_001_mdrmd(y: &[Float]) -> Catch22Result<Float> {
    dn_outliner_include_np_001_mdrmd(y, 1)
}

fn dn_outliner_include_n_001_mdrmd(y: &[Float]) -> Catch22Result<Float> {
    dn_outliner_include_np_001_mdrmd(y, -1)
}

const STEP: usize = 100;

const fn inc() -> f64 {
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

    let max = *y_work.iter().max().unwrap();

    if max < 0.01 {
        return Ok(0.0);
    }

    let threshold = ((max * STEP) + 1) as usize;

    let mut fbi = None; // 一番最初に見つけたhigh_value_count == 0のときのj
    let mut mj = 0; // high_value_count - 1が2を超えた最初のj
    let mut ms_dti4 = Vec::with_capacity(threshold);
    for j in 0..threshold {
        let mut indices = y_work.iter().enumerate().filter_map(|(idx, &&x)|(x > j as f64 * inc()).then_some(idx + 1)).collect_vec();
        let high_value_count = indices.len();
        let indices_diff = indices.iter().zip(indices.iter().skip(1)).map(|(&x, &y)| (y - x) as f64).collect_vec();

        if high_value_count == 0 && fbi.is_none() {
            fbi = Some(j);
        }
        if (high_value_count - 1) * (STEP as f64) / tot > 2.0 {
            mj = j;
        }
        ms_dti4.push(median(&indices_diff));
    }
    let fbi = fbi.unwrap_or(threshold - 1);

    let trim_limit = mj.max(fbi);
    Ok(median(&ms_dti4[0..trim_limit + 1]))
}