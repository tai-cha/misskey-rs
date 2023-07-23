#[cfg(not(feature = "13-14-0"))]
use serde::{Deserialize, Serialize};

#[cfg(feature = "13-14-0")]
pub mod create;
#[cfg(feature = "13-14-0")]
pub mod list;

#[cfg(not(feature = "13-14-0"))]
#[cfg_attr(docsrs, doc(cfg(not(feature = "13-14-0"))))]
#[derive(Serialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {}

#[cfg(not(feature = "13-14-0"))]
#[cfg_attr(docsrs, doc(cfg(not(feature = "13-14-0"))))]
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub code: String,
}

#[cfg(not(feature = "13-14-0"))]
impl misskey_core::Request for Request {
    type Response = Response;
    const ENDPOINT: &'static str = "admin/invite";
}

#[cfg(not(feature = "13-14-0"))]
#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        client.admin.test(Request::default()).await;
    }
}
