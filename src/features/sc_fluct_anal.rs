use crate::error::{Catch22Error, Catch22Result};
use crate::primitive::Float;
use crate::stat::{linear_regression, linear_regression_square_error};
use crate::utility::any_nan;
use itertools::Itertools;
use unwrap_ord::UnwrapOrd;

enum Method {
    Dfa,
    Rsrangefit,
}

pub fn sc_fluct_anal_2_dfa_50_1_2_logi_prop_r1(values: &[Float]) -> Catch22Result<Float> {
    sc_fluct_anal_2_50_1_logi_prop_r1(values, 2, Method::Dfa)
}

pub fn sc_fluct_anal_2_rsrangefit_50_1_logi_prop_r1(values: &[Float]) -> Catch22Result<Float> {
    sc_fluct_anal_2_50_1_logi_prop_r1(values, 1, Method::Rsrangefit)
}

fn sc_fluct_anal_2_50_1_logi_prop_r1(
    values: &[Float],
    lag: usize,
    how: Method,
) -> Catch22Result<Float> {
    if values.is_empty() {
        return Err(Catch22Error::EmptyInput);
    }
    if any_nan(values) {
        return Ok(Float::NAN);
    }

    let lin_low = 5.0_f64.ln();
    let lin_high = ((values.len() / 2) as Float).ln();

    const N_TAU_STEPS: usize = 50;
    let tau_step = (lin_high - lin_low) / ((N_TAU_STEPS - 1) as Float);

    let tau = (0..N_TAU_STEPS)
        .map(|i| (lin_low + (i as Float) * tau_step).exp().round() as usize)
        .dedup()
        .collect_vec();

    if tau.len() < 12 {
        return Ok(0.0);
    }

    let value_cumsum = values
        .iter()
        .step_by(lag)
        .scan(0.0, |sum, v| {
            *sum += v;
            Some(*sum)
        })
        .collect_vec();

    let x_reg = (1..=(*tau.last().unwrap()))
        .map(|x| x as Float)
        .collect_vec();

    let fs = tau
        .iter()
        .map(|&t| {
            let n_buffer = value_cumsum.len() / t;

            let f = (0..n_buffer)
                .map(|j| {
                    let (a, b) = linear_regression(
                        x_reg.iter().take(t),
                        value_cumsum.iter().skip(j * t).take(t),
                    );

                    let vs = value_cumsum
                        .iter()
                        .skip(j * t)
                        .take(t)
                        .enumerate()
                        .map(|(k, c)| c - (a * (k + 1) as Float + b));

                    match how {
                        Method::Rsrangefit => {
                            let (min, max) = vs.minmax().into_option().unwrap();
                            (max - min).powi(2)
                        }
                        Method::Dfa => vs.map(|v| v.powi(2)).sum::<Float>(),
                    }
                })
                .sum::<Float>();

            match how {
                Method::Rsrangefit => (f / n_buffer as Float).sqrt(),
                Method::Dfa => (f / (n_buffer * t) as Float).sqrt(),
            }
        })
        .collect_vec();

    const MIN_POINTS: usize = 6;
    let log_tau = tau.iter().map(|&t| (t as Float).ln()).collect_vec();
    let log_fs = fs.iter().map(|&f| f.ln()).collect_vec();

    let mut sserr = vec![];
    for i in MIN_POINTS..(tau.len() - MIN_POINTS) {
        sserr.push(
            linear_regression_square_error(
                &log_tau[0..i],
                &log_fs[0..i].iter().map(|&t| t as Float).collect_vec(),
            )
            .sqrt()
                + linear_regression_square_error(
                    &log_tau[(i - 1)..],
                    &log_fs[(i - 1)..].iter().map(|&t| t as Float).collect_vec(),
                )
                .sqrt(),
        );
    }

    let first_min_index = sserr
        .iter()
        .position_min_by_key(|&&x| UnwrapOrd(x))
        .unwrap()
        + MIN_POINTS
        - 1;

    Ok((first_min_index as Float + 1.0) / (tau.len() as Float))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utility::load_test_data;
    use approx::assert_abs_diff_eq;

    // other test cases...

    #[test]
    fn test_sc_fluct_anal_2_rsrangefit_50_1_logi_prop_r1_same_original() {
        let numbers = load_test_data();

        assert_abs_diff_eq!(
            sc_fluct_anal_2_rsrangefit_50_1_logi_prop_r1(&numbers).unwrap(),
            0.29545454545455,
            epsilon = 1e-12
        );
    }

    #[test]
    fn test_sc_fluct_anal_2_dfa_50_1_2_logi_prop_r1_same_original() {
        let numbers = load_test_data();

        assert_abs_diff_eq!(
            sc_fluct_anal_2_dfa_50_1_2_logi_prop_r1(&numbers).unwrap(),
            0.75000000000000,
            epsilon = 1e-12
        );
    }
}
