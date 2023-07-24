use crate::primitive::Float;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Catch22Error {
    #[error("Empty input")]
    EmptyInput,
    #[error("Size Over: {0}")]
    SizeOver(usize),
    #[error("Size Under: {len} < {expect_minimum}")]
    SizeUnder { len: usize, expect_minimum: usize },
    #[error("Invalid Parameter: {name} {value}")]
    InvalidFloatParameter { name: &'static str, value: Float },
    #[error("Invalid Parameter: {name} {value}")]
    InvalidUsizeParameter { name: &'static str, value: usize },
}

pub type Catch22Result<T> = Result<T, Catch22Error>;
