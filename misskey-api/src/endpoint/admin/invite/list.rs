use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::model::{
    invite_code::{InviteCode, InviteCodeSortKey, InviteCodeType},
    sort::SortOrder,
};

#[derive(Serialize, Default, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    /// 1 .. 100
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub offset: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub type_: Option<InviteCodeType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option))]
    pub sort: Option<SortOrder<InviteCodeSortKey>>,
}

impl misskey_core::Request for Request {
    type Response = Vec<InviteCode>;
    const ENDPOINT: &'static str = "admin/invite/list";
}

impl_offset_pagination!(Request, InviteCode);

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        client.test(Request::default()).await;
    }

    #[tokio::test]
    async fn request_with_limit() {
        let client = TestClient::new();
        client
            .test(Request {
                limit: Some(100),
                offset: None,
                type_: None,
                sort: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_offset() {
        let client = TestClient::new();
        client
            .test(Request {
                limit: None,
                offset: Some(5),
                type_: None,
                sort: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_type() {
        use crate::model::invite_code::InviteCodeType;

        let client = TestClient::new();

        client
            .test(Request {
                limit: None,
                offset: None,
                type_: Some(InviteCodeType::Unused),
                sort: None,
            })
            .await;
        client
            .test(Request {
                limit: None,
                offset: None,
                type_: Some(InviteCodeType::Used),
                sort: None,
            })
            .await;
        client
            .test(Request {
                limit: None,
                offset: None,
                type_: Some(InviteCodeType::Expired),
                sort: None,
            })
            .await;
        client
            .test(Request {
                limit: None,
                offset: None,
                type_: Some(InviteCodeType::All),
                sort: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_sort() {
        use crate::model::{invite_code::InviteCodeSortKey, sort::SortOrder};

        let client = TestClient::new();

        client
            .test(Request {
                limit: None,
                offset: None,
                type_: None,
                sort: Some(SortOrder::Ascending(InviteCodeSortKey::CreatedAt)),
            })
            .await;
        client
            .test(Request {
                limit: None,
                offset: None,
                type_: None,
                sort: Some(SortOrder::Descending(InviteCodeSortKey::CreatedAt)),
            })
            .await;
        client
            .test(Request {
                limit: None,
                offset: None,
                type_: None,
                sort: Some(SortOrder::Ascending(InviteCodeSortKey::UsedAt)),
            })
            .await;
        client
            .test(Request {
                limit: None,
                offset: None,
                type_: None,
                sort: Some(SortOrder::Descending(InviteCodeSortKey::UsedAt)),
            })
            .await;
    }
}
