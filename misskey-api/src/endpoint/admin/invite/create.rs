use crate::model::invite_code::InviteCode;

use chrono::{DateTime, Utc};
use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub expires_at: Option<DateTime<Utc>>,
}

impl misskey_core::Request for Request {
    type Response = Vec<InviteCode>;
    const ENDPOINT: &'static str = "admin/invite/create";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();

        client
            .admin
            .test(Request {
                count: None,
                expires_at: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_options() {
        let client = TestClient::new();

        client
            .admin
            .test(Request {
                count: Some(100),
                expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(1)),
            })
            .await;
    }
}
