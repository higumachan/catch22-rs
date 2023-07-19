use thiserror::Error;

#[derive(Debug, Error)]
pub enum Catch22Error {
    #[error(display = "Empty input")]
    EmptyInput,
}

pub type Catch22Result<T> = Result<T, Catch22Error>;