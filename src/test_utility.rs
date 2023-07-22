use crate::primitive::Float;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn load_test_data() -> Vec<Float> {
    let input_file = File::open("./test_data/test.txt").unwrap();

    let buf_reader = BufReader::new(input_file);

    buf_reader
        .lines()
        .map(|l| l.unwrap().parse::<f64>().unwrap())
        .collect_vec()
}
