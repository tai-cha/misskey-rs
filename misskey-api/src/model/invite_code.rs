use std::fmt::{self, Display};

use crate::model::{id::Id, user::User};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InviteCode {
    pub id: Id<InviteCode>,
    pub code: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<User>,
    pub used_by: Option<User>,
    pub used_at: Option<DateTime<Utc>>,
    pub used: bool,
}

impl_entity!(InviteCode);

#[derive(Serialize, PartialEq, Eq, Clone, Debug, Copy)]
#[serde(rename_all = "camelCase")]
pub enum InviteCodeType {
    Unused,
    Used,
    Expired,
    All,
}

#[derive(Debug, Error, Clone)]
#[error("invalid user origin")]
pub struct ParseInviteCodeTypeError {
    _priv: (),
}

impl std::str::FromStr for InviteCodeType {
    type Err = ParseInviteCodeTypeError;

    fn from_str(s: &str) -> Result<InviteCodeType, Self::Err> {
        match s {
            "unused" | "Unused" => Ok(InviteCodeType::Unused),
            "used" | "Used" => Ok(InviteCodeType::Used),
            "expired" | "Expired" => Ok(InviteCodeType::Expired),
            "all" | "All" => Ok(InviteCodeType::All),
            _ => Err(ParseInviteCodeTypeError { _priv: () }),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum InviteCodeSortKey {
    CreatedAt,
    UsedAt,
}

impl Display for InviteCodeSortKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InviteCodeSortKey::CreatedAt => f.write_str("createdAt"),
            InviteCodeSortKey::UsedAt => f.write_str("usedAt"),
        }
    }
}

#[derive(Debug, Error, Clone)]
#[error("invalid sort key")]
pub struct ParseInviteCodeSortKeyError {
    _priv: (),
}

impl std::str::FromStr for InviteCodeSortKey {
    type Err = ParseInviteCodeSortKeyError;

    fn from_str(s: &str) -> Result<InviteCodeSortKey, Self::Err> {
        match s {
            "createdAt" | "CreatedAt" => Ok(InviteCodeSortKey::CreatedAt),
            "usedAt" | "UsedAt" => Ok(InviteCodeSortKey::UsedAt),
            _ => Err(ParseInviteCodeSortKeyError { _priv: () }),
        }
    }
}
