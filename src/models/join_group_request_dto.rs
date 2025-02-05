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

/// JoinGroupRequestDto : Class JoinGroupRequestDto.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JoinGroupRequestDto {
    /// Gets or sets the group identifier.
    #[serde(rename = "GroupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<uuid::Uuid>,
}

impl JoinGroupRequestDto {
    /// Class JoinGroupRequestDto.
    pub fn new() -> JoinGroupRequestDto {
        JoinGroupRequestDto { group_id: None }
    }
}
