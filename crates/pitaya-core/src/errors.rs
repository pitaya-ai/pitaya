use thiserror::Error;

#[derive(Debug, Clone, Error, serde::Serialize, serde::Deserialize)]
#[serde(tag = "code", content = "message")]
#[non_exhaustive]
pub enum PitayaError {
    #[error("invalid state transition: {0}")]
    InvalidTransition(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("internal error")]
    Internal,
}
