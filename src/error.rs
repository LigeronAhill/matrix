#[derive(Debug)]
pub enum Error {
    InvalidMatrixSize,
}
pub type Result<T> = core::result::Result<T, Error>;
