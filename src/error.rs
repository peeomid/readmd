use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum ReadmdError {
    #[error("{0}")]
    Message(String),
    #[error("failed to read {path}: {source}")]
    ReadFile {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("failed to write {path}: {source}")]
    WriteFile {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("failed to parse TOML config {path}: {source}")]
    ParseToml {
        path: PathBuf,
        source: toml::de::Error,
    },
    #[error("unknown theme: {0}")]
    UnknownTheme(String),
}

pub type Result<T> = std::result::Result<T, ReadmdError>;
