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

/// DefaultDirectoryBrowserInfoDto : Default directory browser info.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DefaultDirectoryBrowserInfoDto {
    /// Gets or sets the path.
    #[serde(
        rename = "Path",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub path: Option<Option<String>>,
}

impl DefaultDirectoryBrowserInfoDto {
    /// Default directory browser info.
    pub fn new() -> DefaultDirectoryBrowserInfoDto {
        DefaultDirectoryBrowserInfoDto { path: None }
    }
}