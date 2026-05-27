use serde::{Deserialize, Serialize};

use crate::{EngineStatusDto, MeetingId, PitayaError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMeeting {
    pub meeting_id: MeetingId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum Query {
    GetStatus(GetStatus),
    GetMeeting(GetMeeting),
}

pub type QueryResult<T> = Result<T, PitayaError>;

pub type StatusQueryResult = QueryResult<EngineStatusDto>;
