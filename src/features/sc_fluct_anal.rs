use crate::error::Catch22Result;
use crate::primitive::Float;

pub fn sc_fluct_anal_2_rsrangefit_50_1_logi_prop_r1(values: &[Float]) -> Catch22Result<Float> {
    todo!()
}

pub fn sc_fluct_anal_2_dfa_50_1_2_logi_prop_r1(values: &[Float]) -> Catch22Result<Float> {
    todo!()
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
