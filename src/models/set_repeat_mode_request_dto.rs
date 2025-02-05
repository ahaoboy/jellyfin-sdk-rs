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

/// SetRepeatModeRequestDto : Class SetRepeatModeRequestDto.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SetRepeatModeRequestDto {
    #[serde(rename = "Mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<models::GroupRepeatMode>,
}

impl SetRepeatModeRequestDto {
    /// Class SetRepeatModeRequestDto.
    pub fn new() -> SetRepeatModeRequestDto {
        SetRepeatModeRequestDto { mode: None }
    }
}
