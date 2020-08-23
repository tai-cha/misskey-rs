use std::{fmt, sync::Arc};

use crate::error::{Error, Result};

use async_std::sync::Mutex;
use async_tungstenite::async_std::{connect_async, ConnectStream};
use async_tungstenite::tungstenite::{error::Error as WsError, Message as WsMessage};
use async_tungstenite::WebSocketStream;
use futures::future::{self, Future, FutureExt, Ready};
use futures::sink::SinkExt;
use futures::stream::{self, SplitSink, SplitStream, Stream, StreamExt};
use serde::{de::DeserializeOwned, Serialize};
use url::Url;

type RawStream = SplitStream<WebSocketStream<ConnectStream>>;
type FilterFn = fn(<RawStream as Stream>::Item) -> Ready<Option<Result<String>>>;
type InnerStream = stream::FilterMap<RawStream, Ready<Option<Result<String>>>, FilterFn>;

/// Receiver channel that communicates with Misskey
pub struct WebSocketReceriver(InnerStream);

impl fmt::Debug for WebSocketReceriver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("WebSocketReceriver").finish()
    }
}

type MapFn<T> = fn(Option<<InnerStream as Stream>::Item>) -> Result<T>;
pub struct RecvJson<'a, T>(future::Map<stream::Next<'a, InnerStream>, MapFn<T>>);

impl<T> Future for RecvJson<'_, T> {
    type Output = Result<T>;
    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Self::Output> {
        std::pin::Pin::new(&mut self.0).poll(cx)
    }
}

impl WebSocketReceriver {
    fn new(stream: RawStream) -> Self {
        fn filter(res: <RawStream as Stream>::Item) -> Ready<Option<Result<String>>> {
            future::ready(match res {
                Ok(WsMessage::Text(t)) => Some(Ok(t)),
                Ok(WsMessage::Ping(_)) => None,
                Ok(WsMessage::Pong(_)) => None,
                Ok(WsMessage::Close(_)) => None,
                Ok(m) => Some(Err(Error::UnexpectedMessage(m))),
                Err(e) => Some(Err(e.into())),
            })
        }

        let filter: FilterFn = filter;
        WebSocketReceriver(stream.filter_map(filter))
    }

    // using concrete type here because impl trait failed to infer appropriate lifetime...
    pub fn recv_json<T: DeserializeOwned>(&mut self) -> RecvJson<'_, T> {
        fn map<T: DeserializeOwned>(opt: Option<Result<String>>) -> Result<T> {
            match opt {
                Some(Ok(t)) => serde_json::from_str(&t).map_err(Into::into),
                Some(Err(e)) => Err(e),
                None => Err(WsError::ConnectionClosed.into()),
            }
        }

        let map: MapFn<T> = map;
        RecvJson(self.0.next().map(map))
    }
}

/// Sender channel that communicates with Misskey
pub struct WebSocketSender(SplitSink<WebSocketStream<ConnectStream>, WsMessage>);

impl fmt::Debug for WebSocketSender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("WebSocketSender").finish()
    }
}

impl WebSocketSender {
    pub async fn send(&mut self, msg: WsMessage) -> Result<()> {
        Ok(self.0.send(msg).await?)
    }

    pub async fn send_json<T: Serialize>(&mut self, x: &T) -> Result<()> {
        self.send(WsMessage::Text(serde_json::to_string(x)?)).await
    }
}

pub type SharedWebSocketSender = Arc<Mutex<WebSocketSender>>;

pub async fn connect_websocket(url: Url) -> Result<(WebSocketSender, WebSocketReceriver)> {
    let (ws, _) = connect_async(url).await?;
    let (sink, stream) = ws.split();
    Ok((WebSocketSender(sink), WebSocketReceriver::new(stream)))
}
