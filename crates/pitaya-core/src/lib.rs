//! Stable shared contracts. No async, no I/O, no side effects.

pub mod chat;
pub mod commands;
pub mod errors;
pub mod events;
pub mod ids;
pub mod queries;
pub mod status;

pub use chat::*;
pub use commands::*;
pub use errors::*;
pub use events::*;
pub use ids::*;
pub use queries::*;
pub use status::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engine_status_roundtrip() {
        let dto = EngineStatusDto::idle();
        let json = serde_json::to_string(&dto).unwrap();
        let back: EngineStatusDto = serde_json::from_str(&json).unwrap();
        assert_eq!(back.state, EngineState::Idle);
    }
}
