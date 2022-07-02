#[derive(Debug)]
pub enum Error{
    Http(String),
    UnknownHttp(String),
    Deserialize(std::io::Error),
    NoDataInMemory
}

pub type Result<T> = std::result::Result<T, Error>;