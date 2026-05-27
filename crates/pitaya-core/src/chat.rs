use serde::{Deserialize, Serialize};

use crate::{ChatThreadId, FolderId, MeetingId};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ChatScope {
    ActiveMeeting,
    SingleMeeting,
    Folder,
    Home,
    Selection,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[non_exhaustive]
pub enum ContextRef {
    Meeting { meeting_id: MeetingId },
    Folder { folder_id: FolderId },
    Note { meeting_id: MeetingId },
    Transcript { meeting_id: MeetingId },
    Today,
    Recent { days: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSend {
    pub thread_id: Option<ChatThreadId>,
    pub scope: ChatScope,
    pub scope_ids: Vec<uuid::Uuid>,
    pub attachments: Vec<ContextRef>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageMeta {
    pub consulted_meetings: u32,
    pub consulted_segments: u32,
    pub retrieval_passes: u32,
    pub incomplete: bool,
}
