use crate::error::{Catch22Error, Catch22Result};
use crate::primitive::Float;
use itertools::Itertools;
use num::ToPrimitive;
use std::convert::TryInto;

pub struct BinData<const B: usize, const BN: usize> {
    pub count: [usize; B],
    pub edges: [Float; BN],
}

pub struct BinDataFlexible {
    pub count: Vec<usize>,
    pub edges: Vec<Float>,
}

pub struct NormalizedBinDataFlexible {
    pub count: Vec<Float>,
    pub edges: Vec<Float>,
}

impl BinDataFlexible {
    pub fn normalize(self, size: usize) -> Catch22Result<NormalizedBinDataFlexible> {
        if size == 0 {
            return Err(Catch22Error::EmptyInput);
        }
        Ok(NormalizedBinDataFlexible {
            count: self
                .count
                .iter()
                .map(|&c| c as Float / size as Float)
                .collect(),
            edges: self.edges,
        })
    }
}

impl NormalizedBinDataFlexible {
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.count.len()
    }
}

pub fn hist_count_flexible(values: &[Float], bins: usize) -> Catch22Result<BinDataFlexible> {
    let (&min, &max) = values
        .iter()
        .minmax()
        .into_option()
        .ok_or(Catch22Error::EmptyInput)?;

    let bin_step = (max - min) / bins as f64;

    let mut count = vec![0usize; bins];
    for v in values {
        let bin_index = ((v - min) / bin_step)
            .to_usize()
            .unwrap()
            .max(0)
            .min(bins - 1);
        count[bin_index] += 1;
    }

    let edges = (0..(bins + 1))
        .map(|i| i as Float * bin_step + min)
        .collect_vec();

    Ok(BinDataFlexible { count, edges })
}

pub fn hist_counts<const B: usize, const BN: usize>(y: &[Float]) -> Catch22Result<BinData<B, BN>> {
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
        .map(|i| i as Float * bin_step + min)
        .collect_vec()
        .try_into()
        .unwrap();

    Ok(BinData { edges, count })
}
