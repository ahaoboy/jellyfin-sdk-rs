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
pub struct DeviceInfo {
    #[serde(
        rename = "Name",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub name: Option<Option<String>>,
    /// Gets or sets the access token.
    #[serde(
        rename = "AccessToken",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub access_token: Option<Option<String>>,
    /// Gets or sets the identifier.
    #[serde(
        rename = "Id",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<Option<String>>,
    /// Gets or sets the last name of the user.
    #[serde(
        rename = "LastUserName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_user_name: Option<Option<String>>,
    /// Gets or sets the name of the application.
    #[serde(
        rename = "AppName",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub app_name: Option<Option<String>>,
    /// Gets or sets the application version.
    #[serde(
        rename = "AppVersion",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub app_version: Option<Option<String>>,
    /// Gets or sets the last user identifier.
    #[serde(rename = "LastUserId", skip_serializing_if = "Option::is_none")]
    pub last_user_id: Option<uuid::Uuid>,
    /// Gets or sets the date last modified.
    #[serde(rename = "DateLastActivity", skip_serializing_if = "Option::is_none")]
    pub date_last_activity: Option<String>,
    #[serde(rename = "Capabilities", skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Box<models::ClientCapabilities>>,
    #[serde(
        rename = "IconUrl",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub icon_url: Option<Option<String>>,
}

impl DeviceInfo {
    pub fn new() -> DeviceInfo {
        DeviceInfo {
            name: None,
            access_token: None,
            id: None,
            last_user_name: None,
            app_name: None,
            app_version: None,
            last_user_id: None,
            date_last_activity: None,
            capabilities: None,
            icon_url: None,
        }
    }
}
