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

/// ValidatePathDto : Validate path object.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidatePathDto {
    /// Gets or sets a value indicating whether validate if path is writable.
    #[serde(rename = "ValidateWritable", skip_serializing_if = "Option::is_none")]
    pub validate_writable: Option<bool>,
    /// Gets or sets the path.
    #[serde(
        rename = "Path",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub path: Option<Option<String>>,
    /// Gets or sets is path file.
    #[serde(
        rename = "IsFile",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_file: Option<Option<bool>>,
}

impl ValidatePathDto {
    /// Validate path object.
    pub fn new() -> ValidatePathDto {
        ValidatePathDto {
            validate_writable: None,
            path: None,
            is_file: None,
        }
    }
}
