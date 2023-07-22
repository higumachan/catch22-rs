use thiserror::Error;

#[derive(Debug, Error)]
pub enum Catch22Error {
    #[error("Empty input")]
    EmptyInput,
    #[error("Size Over")]
    SizeOver,
    #[error("Size Over")]
    SizeUnder { len: usize, expect_minimum: usize },
}

pub type Catch22Result<T> = Result<T, Catch22Error>;
