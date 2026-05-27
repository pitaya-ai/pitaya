use serde::{Deserialize, Serialize};

use crate::{MeetingId, PitayaError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartMeeting {
    pub meeting_id: MeetingId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopMeeting {
    pub meeting_id: MeetingId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum Command {
    StartMeeting(StartMeeting),
    StopMeeting(StopMeeting),
}

pub type CommandResult<T> = Result<T, PitayaError>;
