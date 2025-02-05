use crate::model::{
    id::Id,
    role::{cond_formula_option, Policies, Role, RoleCondFormulaValue, Target},
};

use serde::Serialize;
use typed_builder::TypedBuilder;

#[derive(Serialize, Debug, Clone, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[builder(doc)]
pub struct Request {
    pub role_id: Id<Role>,
    #[builder(default, setter(into))]
    pub name: String,
    #[builder(default, setter(into))]
    pub description: String,
    #[builder(default, setter(strip_option, into))]
    pub color: Option<String>,
    #[cfg(feature = "13-4-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-4-0")))]
    #[builder(default, setter(strip_option, into))]
    pub icon_url: Option<String>,
    #[builder(default, setter(into))]
    pub target: Target,
    #[serde(with = "cond_formula_option")]
    #[builder(default, setter(strip_option, into))]
    pub cond_formula: Option<RoleCondFormulaValue>,
    #[builder(default, setter(into))]
    pub is_public: bool,
    #[builder(default, setter(into))]
    pub is_moderator: bool,
    #[builder(default, setter(into))]
    pub is_administrator: bool,
    #[cfg(feature = "13-12-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-12-0")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default, setter(strip_option, into))]
    pub is_explorable: Option<bool>,
    #[cfg(feature = "13-4-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-4-0")))]
    #[builder(default, setter(into))]
    pub as_badge: bool,
    #[builder(default, setter(into))]
    pub can_edit_members_by_moderator: bool,
    #[cfg(feature = "13-10-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "13-10-0")))]
    #[builder(default, setter(into))]
    pub display_order: i64,
    #[builder(default, setter(into))]
    pub policies: Policies,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "admin/roles/update";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::model::role::{Policies, PolicyValue, Priority, RoleCondFormulaValue, Target};
    use crate::test::{ClientExt, TestClient};
    use chrono::Duration;

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let role = client
            .admin
            .test(crate::endpoint::admin::roles::create::Request::default())
            .await;

        client
            .admin
            .test(Request {
                role_id: role.id,
                name: String::new(),
                description: String::new(),
                color: None,
                #[cfg(feature = "13-4-0")]
                icon_url: None,
                target: Target::Manual,
                cond_formula: None,
                is_public: false,
                is_moderator: false,
                is_administrator: false,
                #[cfg(feature = "13-12-0")]
                is_explorable: None,
                #[cfg(feature = "13-4-0")]
                as_badge: true,
                can_edit_members_by_moderator: false,
                #[cfg(feature = "13-10-0")]
                display_order: 0,
                policies: Policies::default(),
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_options() {
        let client = TestClient::new();
        #[cfg(feature = "13-4-0")]
        let image_url = client.avatar_url().await;
        let role = client
            .admin
            .test(crate::endpoint::admin::roles::create::Request::default())
            .await;

        client
            .admin
            .test(Request {
                role_id: role.id,
                name: "role".to_string(),
                description: "description".to_string(),
                color: Some("#ff0000".to_string()),
                #[cfg(feature = "13-4-0")]
                icon_url: Some(image_url.to_string()),
                target: Target::Conditional,
                cond_formula: Some(RoleCondFormulaValue::And {
                    values: vec![
                        RoleCondFormulaValue::Or {
                            values: vec![
                                RoleCondFormulaValue::Not {
                                    value: Box::new(RoleCondFormulaValue::IsLocal),
                                },
                                RoleCondFormulaValue::IsRemote,
                            ],
                        },
                        RoleCondFormulaValue::CreatedLessThan {
                            duration: Duration::days(2),
                        },
                        RoleCondFormulaValue::CreatedMoreThan {
                            duration: Duration::minutes(3),
                        },
                        RoleCondFormulaValue::FollowersLessThanOrEq { value: 100 },
                        RoleCondFormulaValue::FollowersMoreThanOrEq { value: 10 },
                        RoleCondFormulaValue::FollowingLessThanOrEq { value: 100 },
                        RoleCondFormulaValue::FollowingMoreThanOrEq { value: 10 },
                    ],
                }),
                is_public: true,
                is_moderator: true,
                is_administrator: true,
                #[cfg(feature = "13-12-0")]
                is_explorable: Some(true),
                #[cfg(feature = "13-4-0")]
                as_badge: true,
                can_edit_members_by_moderator: true,
                #[cfg(feature = "13-10-0")]
                display_order: 1,
                policies: Policies {
                    gtl_available: Some(PolicyValue {
                        use_default: true,
                        priority: Priority::Low,
                        value: false,
                    }),
                    ltl_available: Some(PolicyValue {
                        use_default: false,
                        priority: Priority::Middle,
                        value: true,
                    }),
                    can_public_note: Some(PolicyValue {
                        use_default: false,
                        priority: Priority::High,
                        value: true,
                    }),
                    can_invite: Some(PolicyValue {
                        use_default: false,
                        priority: Priority::High,
                        value: true,
                    }),
                    can_manage_custom_emojis: Some(PolicyValue {
                        use_default: false,
                        priority: Priority::High,
                        value: true,
                    }),
                    #[cfg(feature = "13-10-0")]
                    can_search_notes: Some(PolicyValue {
                        use_default: false,
                        priority: Priority::High,
                        value: true,
                    }),
                    can_hide_ads: Some(PolicyValue {
                        use_default: false,
                        priority: Priority::High,
                        value: true,
                    }),
                    drive_capacity_mb: Some(PolicyValue {
                        use_default: false,
                        priority: Priority::High,
                        value: 1000,
                    }),
                    pin_limit: Some(PolicyValue {
                        use_default: false,
                        priority: Priority::High,
                        value: 100,
                    }),
                    antenna_limit: Some(PolicyValue {
                        use_default: false,
                        priority: Priority::High,
                        value: 10,
                    }),
                    word_mute_limit: Some(PolicyValue {
                        use_default: false,
                        priority: Priority::High,
                        value: 10000,
                    }),
                    webhook_limit: Some(PolicyValue {
                        use_default: false,
                        priority: Priority::High,
                        value: 10,
                    }),
                    clip_limit: Some(PolicyValue {
                        use_default: false,
                        priority: Priority::High,
                        value: 1000,
                    }),
                    note_each_clips_limit: Some(PolicyValue {
                        use_default: false,
                        priority: Priority::High,
                        value: 10000,
                    }),
                    user_list_limit: Some(PolicyValue {
                        use_default: false,
                        priority: Priority::High,
                        value: 100,
                    }),
                    user_each_user_lists_limit: Some(PolicyValue {
                        use_default: false,
                        priority: Priority::High,
                        value: 1000,
                    }),
                    rate_limit_factor: Some(PolicyValue {
                        use_default: false,
                        priority: Priority::High,
                        value: 0.5,
                    }),
                },
            })
            .await;
    }
}
