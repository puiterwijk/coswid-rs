use thiserror::Error;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum Error {
    #[error("CBOR Syntax error at position {0}")]
    CborSyntax(usize),
    #[error("CBOR Semantic error at position {0:?}: {1}")]
    CborSemantic(Option<usize>, String),
    #[error("CBOR value error: {0}")]
    CborValue(String),
    #[error("IO Error: {0}")]
    Io(String),
    #[error("Missing field in wire: {0}")]
    MissingField(String),
    #[error("Invalid field value for {0}: {1}")]
    InvalidField(String, String),
}

impl<T: std::fmt::Display> From<ciborium::de::Error<T>> for Error {
    fn from(de_err: ciborium::de::Error<T>) -> Self {
        match de_err {
            ciborium::de::Error::Syntax(loc) => Error::CborSyntax(loc),
            ciborium::de::Error::Semantic(loc, desc) => Error::CborSemantic(loc, desc),
            ciborium::de::Error::Io(io_err) => Error::Io(format!("{}", io_err)),
        }
    }
}

impl<T: std::fmt::Display> From<ciborium::ser::Error<T>> for Error {
    fn from(ser_err: ciborium::ser::Error<T>) -> Self {
        match ser_err {
            ciborium::ser::Error::Value(desc) => Error::CborValue(desc),
            ciborium::ser::Error::Io(io_err) => Error::Io(format!("{}", io_err)),
        }
    }
}

impl Error {
    pub(crate) fn from_field(fieldname: String, val_err: ciborium::value::Error) -> Self {
        match val_err {
            ciborium::value::Error::Custom(desc) => Error::InvalidField(fieldname, desc)
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
