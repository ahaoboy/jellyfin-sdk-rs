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

/// UpdateUserPassword : The update user password request body.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateUserPassword {
    /// Gets or sets the current sha1-hashed password.
    #[serde(
        rename = "CurrentPassword",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub current_password: Option<Option<String>>,
    /// Gets or sets the current plain text password.
    #[serde(
        rename = "CurrentPw",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub current_pw: Option<Option<String>>,
    /// Gets or sets the new plain text password.
    #[serde(
        rename = "NewPw",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub new_pw: Option<Option<String>>,
    /// Gets or sets a value indicating whether to reset the password.
    #[serde(rename = "ResetPassword", skip_serializing_if = "Option::is_none")]
    pub reset_password: Option<bool>,
}

impl UpdateUserPassword {
    /// The update user password request body.
    pub fn new() -> UpdateUserPassword {
        UpdateUserPassword {
            current_password: None,
            current_pw: None,
            new_pw: None,
            reset_password: None,
        }
    }
}
