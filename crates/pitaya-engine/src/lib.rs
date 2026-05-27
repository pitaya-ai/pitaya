//! Runtime orchestration for Pitaya. Started inside `pitaya-desktop`.

mod engine_handle;
pub mod ipc_server;

pub use engine_handle::EngineHandle;

/// Cooperative engine runtime (stub until P1 task graph).
pub async fn run() {
    tracing::info!("pitaya-engine started (harness stub)");
    std::future::pending::<()>().await;
}
