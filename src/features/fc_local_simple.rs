use crate::error::{Catch22Error, Catch22Result};
use crate::features::co_auto_corr::co_firstzero;
use crate::primitive::Float;
use crate::utility::{any_nan, mean, stddev};
use itertools::Itertools;

pub fn fc_local_simple_mean1_tauresrat(values: &[Float]) -> Catch22Result<Float> {
    fc_local_simple_mean_tauresrat(values, 1)
}

pub fn fc_localsimple_mean3_stderr(values: &[Float]) -> Catch22Result<Float> {
    fc_local_simple_mean_stderr(values, 3)
}

fn fc_local_simple_mean_tauresrat(values: &[Float], train_length: usize) -> Catch22Result<Float> {
    if any_nan(values) {
        return Ok(Float::NAN);
    }

    let res = train_mean(values, train_length)?;
    let res_ac_1st_z = co_firstzero(&res, values.len() - train_length)?;
    let y_ac_1st_x = co_firstzero(values, values.len())?;

    Ok(res_ac_1st_z as Float / y_ac_1st_x as Float)
}

fn fc_local_simple_mean_stderr(values: &[Float], train_length: usize) -> Catch22Result<Float> {
    if any_nan(values) {
        return Ok(Float::NAN);
    }

    let res = train_mean(values, train_length)?;

    Ok(stddev(&res).unwrap())
}

fn train_mean(values: &[Float], train_length: usize) -> Catch22Result<Vec<Float>> {
    if train_length == 0 {
        return Err(Catch22Error::InvalidUsizeParameter {
            name: "train_length",
            value: train_length,
        });
    }

    Ok((0..values.len() - train_length)
        .map(|i| {
            let train_mean = mean(&values[i..i + train_length]).unwrap();
            values[i + train_length] - train_mean
        })
        .collect_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utility::load_test_data;
    use approx::assert_abs_diff_eq;

    // other test cases...

    #[test]
    fn test_fc_localsimple_mean1_tauresrat_same_original() {
        let values = load_test_data();

        assert_abs_diff_eq!(
            fc_local_simple_mean1_tauresrat(&values).unwrap(),
            0.84782608695652,
            epsilon = 1e-14
        );
    }

    #[test]
    fn test_fc_localsimple_mean3_stderr_same_original() {
        let numbers = load_test_data();

        assert_abs_diff_eq!(
            fc_localsimple_mean3_stderr(&numbers).unwrap(),
            0.08029384289851,
            epsilon = 1e-7
        );
    }
}
