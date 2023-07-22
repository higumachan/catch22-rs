use crate::error::{Catch22Error, Catch22Result};
use crate::fft::{fft, twiddles};
use crate::hist_count::hist_count_flexible;
use crate::primitive::Float;
use crate::utility::{any_nan, mean, mean_iter, nextpow2, stddev};
use itertools::Itertools;
use num::{Complex, Zero};
use std::f64::consts::E;
use unwrap_ord::UnwrapOrd;

pub fn co_embed2_dist_tau_d_expfit_meandiff(y: &[Float]) -> Catch22Result<Float> {
    if any_nan(y) {
        return Ok(Float::NAN);
    }

    let tau = co_firstzero(y, y.len())?.min(y.len() / 10);

    let d = (0..(y.len() - tau - 1))
        .map(|i| ((y[i + 1] - y[i]).powi(2) + (y[i + tau] - y[i + tau + 1]).powi(2)).sqrt())
        .collect_vec();
    let dl = mean_iter(d.iter().copied()).unwrap();

    let bin_data = hist_count_flexible(&d, num_bins_auto(&d).unwrap())?;
    let bin_data = bin_data.normalize(d.len())?;

    Ok(mean_iter((0..bin_data.len()).map(|i| {
        let exp = (-(bin_data.edges[i] + bin_data.edges[i + 1]) * 0.5 / dl).exp() / dl;
        (bin_data.count[i] - exp.max(0.0)).abs()
    }))
    .unwrap())
}

pub fn co_f1ecac(values: &[Float]) -> Catch22Result<Float> {
    if any_nan(values) {
        return Ok(Float::NAN);
    }

    let auto_corr = &co_autocorrs(values)?[..values.len()];

    let threshold = 1.0 / E;

    Ok(auto_corr
        .iter()
        .enumerate()
        .zip(auto_corr.iter().skip(1))
        .find_map(|((i, &a), &an)| {
            (an < threshold).then(|| {
                let m = an - a;
                let dy = threshold - a;
                let dx = dy / m;
                (i as Float) + dx
            })
        })
        .unwrap_or(values.len() as Float))
}

fn co_autocorrs(values: &[Float]) -> Catch22Result<Vec<Float>> {
    let mean = mean(values).unwrap();
    let n_fft = nextpow2(values.len())
        .and_then(|s| s.checked_shl(1))
        .ok_or(Catch22Error::SizeOver)?;

    let mut f = values
        .iter()
        .map(|y| Complex::<Float>::new(y - mean, 0.0))
        .chain((values.len()..n_fft).map(|_| Complex::zero()))
        .collect_vec();

    let tw = twiddles(n_fft);
    fft(&mut f, n_fft, &tw);
    let mut f = f.iter().map(|x| x * x.conj()).collect_vec();
    fft(&mut f, n_fft, &tw);

    let divisor = f.first().unwrap();

    Ok(f.iter().map(|x| (x / divisor).re).collect_vec())
}

fn co_firstzero(y: &[Float], max_tau: usize) -> Catch22Result<usize> {
    let ac = co_autocorrs(y)?;

    let (index, _) = ac
        .iter()
        .enumerate()
        .find_position(|&(i, &x)| !(x > 0.0 && i < max_tau))
        .unwrap();

    Ok(index)
}

pub fn num_bins_auto(values: &[Float]) -> Option<usize> {
    let (min, max) = values
        .iter()
        .copied()
        .minmax_by_key(|&a| UnwrapOrd(a))
        .into_option()?;

    let s = stddev(values)?;

    Some(if s < 0.001 {
        0
    } else {
        let size_pow = (values.len() as Float).powf(1.0 / 3.0);
        ((max - min) / (3.5 * s / size_pow)).ceil() as usize
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utility::load_test_data;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_co_embed2_dist_tau_d_expfit_meandiff() {
        let numbers = load_test_data();

        assert_abs_diff_eq!(
            co_embed2_dist_tau_d_expfit_meandiff(&numbers).unwrap(),
            7.1350786087885,
            epsilon = 1e-5,
        )
    }

    #[test]
    fn test_co_f1ecac_same_original() {
        let numbers = load_test_data();

        assert_abs_diff_eq!(
            co_f1ecac(&numbers).unwrap(),
            32.50260547693647,
            epsilon = 1e-11
        );
    }
}
