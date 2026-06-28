use thiserror::Error;

/// Library error type.
///
/// Define variants that match your domain errors.
/// This error implements `From` for common conversions.
#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("processing failed: {0}")]
    Processing(String),

    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Processing(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display_invalid_input() {
        let err = Error::InvalidInput("bad data".into());
        assert_eq!(err.to_string(), "invalid input: bad data");
    }

    #[test]
    fn error_display_processing() {
        let err = Error::Processing("something went wrong".into());
        assert_eq!(err.to_string(), "processing failed: something went wrong");
    }
}
