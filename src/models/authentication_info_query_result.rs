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
pub struct AuthenticationInfoQueryResult {
    /// Gets or sets the items.
    #[serde(
        rename = "Items",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub items: Option<Option<Vec<models::AuthenticationInfo>>>,
    /// Gets or sets the total number of records available.
    #[serde(rename = "TotalRecordCount", skip_serializing_if = "Option::is_none")]
    pub total_record_count: Option<i32>,
    /// Gets or sets the index of the first record in Items.
    #[serde(rename = "StartIndex", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
}

impl AuthenticationInfoQueryResult {
    pub fn new() -> AuthenticationInfoQueryResult {
        AuthenticationInfoQueryResult {
            items: None,
            total_record_count: None,
            start_index: None,
        }
    }
}
