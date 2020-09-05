use std::hash::Hash;
use std::sync::Arc;

use crate::broker::channel::{ChannelPongSender, ResponseSender, ResponseStreamSender};
use crate::error::Error;
use crate::model::{ApiRequestId, ChannelId};

#[cfg(all(not(feature = "tokio-runtime"), feature = "async-std-runtime"))]
use async_std::sync::RwLock;
use derive_more::{Display, FromStr};
use misskey_core::model::ApiResult;
use misskey_core::streaming::SubNoteId;
use serde_json::Value;
#[cfg(all(feature = "tokio-runtime", not(feature = "async-std-runtime")))]
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Clone, PartialEq, Eq, Hash, FromStr, Debug, Display, Copy)]
pub(crate) struct BroadcastId(pub Uuid);

impl BroadcastId {
    pub fn new() -> Self {
        BroadcastId(Uuid::new_v4())
    }
}

#[derive(Debug)]
pub(crate) enum BrokerControl {
    HandleApiResponse {
        id: ApiRequestId,
        sender: ResponseSender<ApiResult<Value>>,
    },
    Connect {
        id: ChannelId,
        name: &'static str,
        sender: ResponseStreamSender<Value>,
        pong: ChannelPongSender,
    },
    Disconnect {
        id: ChannelId,
    },
    SubNote {
        id: SubNoteId,
        sender: ResponseStreamSender<Value>,
    },
    UnsubNote {
        id: SubNoteId,
    },
    StartBroadcast {
        id: BroadcastId,
        type_: &'static str,
        sender: ResponseStreamSender<Value>,
    },
    StopBroadcast {
        id: BroadcastId,
    },
}

#[derive(Debug, Clone)]
pub(crate) enum BrokerState {
    Working,
    Exited,
    Dead(Error),
}

impl BrokerState {
    pub fn dead(&self) -> Option<&Error> {
        match self {
            BrokerState::Working => None,
            // TODO: clearify the guarantee that no one asks for `BrokerState` after broker is dead
            BrokerState::Exited => panic!("asked if broker is dead while it is already exited"),
            BrokerState::Dead(e) => Some(e),
        }
    }
}

pub(crate) type SharedBrokerState = Arc<RwLock<BrokerState>>;
