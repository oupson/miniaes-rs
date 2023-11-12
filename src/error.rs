#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid input length")]
    InvalidInputLength,
}

pub type Result<T> = std::result::Result<T, Error>;
