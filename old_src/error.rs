#[derive(Debug)]
pub enum Error {
    Initialization,
    InvalidInput,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Self::Initialization => "failed to initialize the application",
            Self::InvalidInput => "invalid input",
        }
    }
}
