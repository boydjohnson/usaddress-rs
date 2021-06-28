#[derive(Debug)]
pub enum Error {
    CrfsError(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(other: std::io::Error) -> Self {
        Error::CrfsError(other)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CrfsError(e) => write!(f, "USAddress Error: {}", e),
        }
    }
}
