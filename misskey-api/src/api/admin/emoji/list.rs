use crate::model::emoji::{Emoji, EmojiId};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<EmojiId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_id: Option<EmojiId>,
}

impl misskey_core::Request for Request {
    type Response = Vec<Emoji>;
    const ENDPOINT: &'static str = "admin/emoji/list";
}
