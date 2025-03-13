use thisError;

#[derive(thisError::Error, Debug)]
pub enum Error {
    Generic(String),
    IO(std::io::Error),
}

pub type Results<T> = core::result::Result<T, Error>;
