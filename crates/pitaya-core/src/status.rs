use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
#[serde(rename_all = "snake_case")]
pub enum EngineState {
    Idle,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
pub struct EngineStatusDto {
    pub state: EngineState,
    pub version: String,
}

impl EngineStatusDto {
    pub fn idle() -> Self {
        Self {
            state: EngineState::Idle,
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}
