use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use catch22_rs::features::dn_outlier_include_n_001_mdrmd::{
    dn_outliner_include_n_001_mdrmd, dn_outliner_include_p_001_mdrmd,
};
use clap::Parser;
use itertools::Itertools;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    input_file: PathBuf,
}

#[allow(clippy::dbg_macro)]
fn main() {
    let cli = Cli::parse();

    let input_file = File::open(cli.input_file).unwrap();

    let buf_reader = BufReader::new(input_file);

    let numbers = buf_reader
        .lines()
        .map(|l| l.unwrap().parse::<f64>().unwrap())
        .collect_vec();

    dbg!(numbers.len());

    let ret_value = dn_outliner_include_p_001_mdrmd(&numbers).unwrap();
    dbg!(ret_value);
    let ret_value = dn_outliner_include_n_001_mdrmd(&numbers).unwrap();
    dbg!(ret_value);
}
