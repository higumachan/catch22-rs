use crate::error::{Catch22Error, Catch22Result};
use crate::primitive::Float;
use crate::utility::{any_nan, entropy, sb_coarsegrain, Alphabet};

pub fn sb_motif_three_quantile_hh(values: &[Float]) -> Catch22Result<Float> {
    if values.is_empty() {
        return Err(Catch22Error::EmptyInput);
    }

    if any_nan(values) {
        return Ok(Float::NAN);
    }

    const ALPHABET_SIZE: usize = 3;
    let labels = sb_coarsegrain(values, ALPHABET_SIZE);

    let mut r1: [Vec<usize>; ALPHABET_SIZE] = Default::default();
    for (i, r) in r1.iter_mut().enumerate() {
        for (j, &l) in labels.iter().enumerate() {
            if l == Alphabet::from_zero_indexed(i) {
                r.push(j);
            }
        }

        if !r.is_empty() && *r.last().unwrap() == values.len() - 1 {
            r.pop();
        }
    }

    let mut hh = 0.0;
    for r in &mut r1 {
        let mut v = [0.0; ALPHABET_SIZE];
        for (j, vj) in v.iter_mut().enumerate() {
            let s = r
                .iter()
                .filter(|&&r| labels[r + 1] == Alphabet::from_zero_indexed(j))
                .count();
            *vj = s as Float / (values.len() as Float - 1.0);
        }
        hh += entropy(&v);
    }

    Ok(hh)
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
