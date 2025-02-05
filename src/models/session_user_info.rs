/*
 * Jellyfin API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 10.8.13
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// SessionUserInfo : Class SessionUserInfo.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SessionUserInfo {
    /// Gets or sets the user identifier.
    #[serde(rename = "UserId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<uuid::Uuid>,
    /// Gets or sets the name of the user.
    #[serde(
        rename = "UserName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub user_name: Option<Option<String>>,
}

impl SessionUserInfo {
    /// Class SessionUserInfo.
    pub fn new() -> SessionUserInfo {
        SessionUserInfo {
            user_id: None,
            user_name: None,
        }
    }
}
