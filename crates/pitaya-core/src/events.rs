use serde::{Deserialize, Serialize};

use crate::{MeetingId, PitayaError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeetingStarted {
    pub meeting_id: MeetingId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeetingEnded {
    pub meeting_id: MeetingId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorRaised {
    pub error: PitayaError,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum Event {
    MeetingStarted(MeetingStarted),
    MeetingEnded(MeetingEnded),
    ErrorRaised(ErrorRaised),
}
