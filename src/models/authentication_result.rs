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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthenticationResult {
    #[serde(rename = "User", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<models::UserDto>>,
    #[serde(rename = "SessionInfo", skip_serializing_if = "Option::is_none")]
    pub session_info: Option<Box<models::SessionInfo>>,
    #[serde(
        rename = "AccessToken",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub access_token: Option<Option<String>>,
    #[serde(
        rename = "ServerId",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub server_id: Option<Option<String>>,
}

impl AuthenticationResult {
    pub fn new() -> AuthenticationResult {
        AuthenticationResult {
            user: None,
            session_info: None,
            access_token: None,
            server_id: None,
        }
    }
}
