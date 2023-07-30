use crate::error::Catch22Result;
use crate::primitive::Float;

pub fn sb_motif_three_quantile_hh(values: &[Float]) -> Catch22Result<Float> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utility::load_test_data;
    use approx::assert_abs_diff_eq;

    // other test cases...

    #[test]
    fn test_sb_motif_three_quantile_hh_same_original() {
        let numbers = load_test_data();

        assert_abs_diff_eq!(
            sb_motif_three_quantile_hh(&numbers).unwrap(),
            1.21058781724385,
            epsilon = 1e-12
        );
    }
}
