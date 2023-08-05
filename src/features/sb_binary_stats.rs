use crate::error::{Catch22Error, Catch22Result};
use crate::primitive::Float;
use crate::utility::{any_nan, mean_iter};

pub fn sb_binarystats_diff_longstretch0(values: &[Float]) -> Catch22Result<Float> {
    if values.is_empty() {
        return Err(Catch22Error::EmptyInput);
    }
    if values.len() <= 1 {
        return Err(Catch22Error::SizeUnder {
            len: values.len(),
            expect_minimum: 2,
        });
    }
    if any_nan(values) {
        return Ok(Float::NAN);
    }

    let (max_strech0, _) = values
        .iter()
        .zip(values.iter().skip(1))
        .map(|(v, nv)| if (nv - v).is_sign_negative() { 0 } else { 1 })
        .enumerate()
        .fold((0, 0), |(max_strech0, last1), (i, v)| {
            if v == 1 || i == (values.len() - 2) {
                let strech0 = i as i32 - last1;
                if strech0 > max_strech0 {
                    (strech0, i as i32)
                } else {
                    (max_strech0, i as i32)
                }
            } else {
                (max_strech0, last1)
            }
        });

    Ok(max_strech0 as Float)
}

pub fn sb_binarystats_mean_longstretch1(values: &[Float]) -> Catch22Result<Float> {
    if values.is_empty() {
        return Err(Catch22Error::EmptyInput);
    }
    if values.len() <= 1 {
        return Err(Catch22Error::SizeUnder {
            len: values.len(),
            expect_minimum: 2,
        });
    }
    if any_nan(values) {
        return Ok(Float::NAN);
    }

    let mean = mean_iter(values.iter().copied()).unwrap();

    let (max_strech1, _) = values
        .iter()
        .take(values.len() - 1) // TODO: check if this is correct
        .map(|v| if *v <= mean { 0 } else { 1 })
        .enumerate()
        .fold((0, 0), |(max_strech1, last0), (i, v)| {
            if v == 0 || i == (values.len() - 2) {
                let strech1 = i as i32 - last0;
                if strech1 > max_strech1 {
                    (strech1, i as i32)
                } else {
                    (max_strech1, i as i32)
                }
            } else {
                (max_strech1, last0)
            }
        });

    Ok(max_strech1 as Float)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utility::load_test_data;
    use approx::assert_abs_diff_eq;

    // other test cases...

    #[test]
    fn test_sb_binarystats_diff_longstretch0_same_original() {
        let numbers = load_test_data();

        assert_abs_diff_eq!(
            sb_binarystats_diff_longstretch0(&numbers).unwrap(),
            83.0,
            epsilon = 1e-12
        );
    }

    #[test]
    fn test_sb_binarystats_mean_longstretch1_same_original() {
        let numbers = load_test_data();

        assert_abs_diff_eq!(
            sb_binarystats_mean_longstretch1(&numbers).unwrap(),
            88.0,
            epsilon = 1e-12
        );
    }
}
