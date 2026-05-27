use thiserror::Error;

#[derive(Debug, Clone, Error, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
#[serde(tag = "code", content = "message")]
pub enum PitayaError {
    #[error("internal error")]
    Internal,
}
