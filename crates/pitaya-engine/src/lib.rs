//! Runtime orchestration for Pitaya. Started inside `pitaya-desktop`.

mod engine_handle;
pub mod ipc_server;

pub use engine_handle::EngineHandle;

/// Cooperative engine runtime (stub until task graph and dispatch API are wired).
pub async fn run() {
    tracing::info!("pitaya-engine started (harness stub)");
    std::future::pending::<()>().await;
}
