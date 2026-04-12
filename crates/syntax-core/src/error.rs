#[derive(Debug, thiserror::Error)]
pub enum SyntaxError {
    #[error("language not supported: {0}")]
    LanguageNotSupported(String),
    #[error("grammar load error: {0}")]
    GrammarLoadError(String),
    #[error("query error: {0}")]
    QueryError(String),
    #[error("parse error")]
    ParseError,
    #[error("document not found")]
    DocumentNotFound,
    #[error("no syntax tree")]
    NoSyntaxTree,
    #[error("invalid edit range")]
    InvalidEditRange,
    #[error("unknown error: {0}")]
    Unknown(String),
}
