use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum EngineState {
    Idle,
    ArmingCapture,
    Recording,
    Paused,
    Stopping,
    PostProcessing,
    ErrorRecoverable,
    ErrorFatal,
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
