use std::sync::Arc;

use pitaya_core::EngineStatusDto;

/// In-process handle for UI and tests. Full task graph wires in P1.
#[derive(Clone)]
pub struct EngineHandle {
    inner: Arc<EngineHandleInner>,
}

struct EngineHandleInner {
    status: EngineStatusDto,
}

impl EngineHandle {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(EngineHandleInner {
                status: EngineStatusDto::idle(),
            }),
        }
    }

    pub fn status(&self) -> EngineStatusDto {
        self.inner.status.clone()
    }
}

impl Default for EngineHandle {
    fn default() -> Self {
        Self::new()
    }
}
