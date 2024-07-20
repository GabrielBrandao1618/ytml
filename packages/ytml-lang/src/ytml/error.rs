use std::error::Error;

#[derive(Debug)]
pub enum YtmlErrorKind {
    Parsing,
}

#[derive(Debug)]
pub struct YtmlError {
    kind: YtmlErrorKind,
}

impl YtmlError {
    pub fn new(kind: YtmlErrorKind) -> Self {
        Self { kind }
    }
}

impl Error for YtmlError {}

impl std::fmt::Display for YtmlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            YtmlErrorKind::Parsing => write!(f, "Parsing error"),
        }
    }
}

pub type YtmlResult<T> = Result<T, YtmlError>;
