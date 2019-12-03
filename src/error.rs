use derive_more::{Display, From};

/// Error type to be used in the crate
#[derive(Display, Debug, From)]
pub enum Error {
    #[display(fmt = "Parse integer failed: {}", _0)]
    ParseInt(std::num::ParseIntError),
    #[display(fmt = "{}", _0)]
    Msg(String),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            _ => None,
        }
    }
}

impl<'a> From<&'a str> for Error {
    fn from(s: &'a str) -> Self {
        Self::Msg(s.into())
    }
}
