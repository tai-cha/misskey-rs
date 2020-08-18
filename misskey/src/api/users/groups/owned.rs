use crate::api::ApiRequest;
use crate::model::user_group::UserGroup;

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {}

impl ApiRequest for Request {
    type Response = Vec<UserGroup>;
    const ENDPOINT: &'static str = "users/groups/owned";
}
